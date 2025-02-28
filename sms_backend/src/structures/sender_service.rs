use std::sync::atomic::AtomicBool;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::{mpsc, oneshot};
use tracing::debug;
use super::progress_monitor::ProgressMonitor;
use super::sender::SimSender;


/* SENDER ACTIONS */

/// All actions that the sender service can perform. Used to ensure request a given 
/// `SenderService` from different threads / tasks are proccessed sequentially to avoid unnecessary complexity.
enum SenderServiceAction {
    SendMessage(SendMessage),
    NewSenderPool(NewSenderPoolRequest),
    ShutDown(oneshot::Sender<()>),
}

/// A struct to represent a request to create a new pool of senders (that will replace the current pool), 
/// with a Sender from a oneshot channel to return the result of the request. 
struct NewSenderPoolRequest {
    new_senders: Vec<SimSender>,
    reply: oneshot::Sender<bool>,
}

/// A struct to represent a request to send a message, with a Sender from a oneshot channel
/// to return the result of the request.
struct SendMessage {
    msg: String,
    reply: oneshot::Sender<bool>,
}


/* SENDER SERVICE */

/// A service that manages a pool of `SimSender` objects, and a progress monitor to inform the system of progress result always.
/// This service can be cloned and shared acros multiple tasks proccessing actions sequentially. 
/// While performing operations on the senders in the pool, without cloning the senders themselves.
#[derive(Clone)]
pub struct SenderService {
    // Where callers send their requests
    actions_tx: mpsc::Sender<SenderServiceAction>,
    pm: Arc<ProgressMonitor>
}

/// Implementation for the SenderService struct
impl SenderService {

    /// Constructs a new `SenderService` with the given senders and progress monitor. 
    /// Initiatlizes a task to handle requests sequentially without having to clone or keep track of the senders themselves.
    /// 
    /// Parameters:
    /// * `senders` - A vector of `SimSender` objects to be used in the pool
    /// * `pm` - A progress monitor to keep track of the progress of the senders
    /// 
    /// Returns: A `SenderService` object that can be used to send messages and get the progress monitor.
    pub async fn new(senders: Vec<SimSender>, pm: Arc<ProgressMonitor>) -> SenderService {

        // This channel holds the available SimSender objects (the "pool").capacity is set to the number of senders, so it never blocks... 
        // and if more senders need to be added in the future, the pool should be replaced with a new one.
        let (sender_pool_tx, sender_pool_rx) = mpsc::channel::<SimSender>(senders.len());

        // Fill the pool!!! with all the given senders.
        for s in senders {
            sender_pool_tx.send(s).await.expect("SenderService channel broken on setup");
        }

        // This channel keeps track of the actions that need to be performed by the by the task loop.
        // capacity is hard coded to 1000, however, this could be changed to a more dynamic value if needed.
        let (actions_tx, mut actions_rx) = mpsc::channel::<SenderServiceAction>(50000);

        let pm_clone = pm.clone();
 
        // Spawn a handling task that will process the actions sequentially, using the above actions channel. 
        tokio::spawn(async move {
            // Notice the sender pool is never exposed to avoid unwanted additions of new senders
            let mut sender_pool_rx: mpsc::Receiver<SimSender> = sender_pool_rx;
            let mut sender_pool_tx = sender_pool_tx;

            let running = AtomicBool::new(true);
            

            // The main handling loop, that supports all actions this struct makes publicly available.
            // WHEN ADDING NEW FUNCTIONS, ENSURE THEY ARE HANDLED HERE
            while let Some(sender_service_action ) = actions_rx.recv().await {
                match sender_service_action { 
                    // sends a message using the next available sender in the pool
                    SenderServiceAction::SendMessage(send_msg_request) => {
                        // ENSURE STILL RUNNING

                        if !running.load(std::sync::atomic::Ordering::SeqCst) {
                            // If the service is shutting down, don't process any more messages
                            let _ = send_msg_request.reply.send(false);
                            continue;
                        }

                        // PROCESS REQUEST
                        SenderService::internal_send_handler(pm_clone.clone(), sender_pool_tx.clone(), &mut sender_pool_rx, send_msg_request).await;

                    }
                    // replaces the current sender pool with a new one
                    SenderServiceAction::NewSenderPool(new_pool_request) => {
                        // ENSURE STILL RUNNING

                        if !running.load(std::sync::atomic::Ordering::SeqCst) {
                            let _ = new_pool_request.reply.send(false);
                            continue;
                        }

                        // PROCESS REQUEST
                        (sender_pool_tx, sender_pool_rx) = mpsc::channel::<SimSender>(new_pool_request.new_senders.len());
                        
                        for ss in new_pool_request.new_senders {
                            sender_pool_tx.send(ss).await.expect("SenderService channel broken on setup");
                        }
                        let _ = new_pool_request.reply.send(true);
                    }
                    // shuts down the service, and waits for all actions to be processed (struct goes bye bye forever)
                    SenderServiceAction::ShutDown(reply) => {
                        // disable all new other actions
                        running.store(false, std::sync::atomic::Ordering::SeqCst);

                        while !actions_rx.is_empty() {
                            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                        }

                        // wait for all senders to be returned to the pool
                        while sender_pool_rx.len() < sender_pool_tx.max_capacity() {
                            tokio::task::yield_now().await;
                        }

                        // send the reply to the caller
                        let _ = reply.send(());
                    }
                }
            }
        });

        SenderService { actions_tx, pm }
    }

    /// Internal function for handling send message requests.
    /// 
    /// Parameters:
    /// * `pm` - The shared progress monitor to keep track of the progress of the senders
    /// * `sender_pool_tx` - The sender pool channel to return the used sender to the pool when finished
    /// * `sender_pool_rx` - The sender pool channel to get the next available sender from
    /// * `action` - The action to be performed (send a message)
    /// 
    /// Returns: A future that will be spawned to handle the send message request, and replys to the action one shot channel when the task finishes.
    async fn internal_send_handler(pm: Arc<ProgressMonitor>,sender_pool_tx: mpsc::Sender<SimSender>, sender_pool_rx: &mut mpsc::Receiver<SimSender>, action: SendMessage) {
        // Acquire a free SimSender from the pool (waits if none is available)
        if let Some(sender) = sender_pool_rx.recv().await {
            // Spawn a task to send the message
            tokio::spawn({ async move {
            let send_time = Instant::now();
            let send_result = sender.send(action.msg).await;
            let elapsed_secs = Instant::now()
                .duration_since(send_time).as_secs();
            if send_result {
                pm.add_message_sent(elapsed_secs);
                let _ = action.reply.send(true);
            } else {
                pm.add_message_failed(elapsed_secs);
                let _ = action.reply.send(false);
            }

            // Return sender to the pool so someone else can use it
            if let Err(e) = sender_pool_tx.send(sender).await {
                debug!("Failed to return sender to pool: {:?}", e);
            }
        
            }});
        } else {
            // No more senders in the pool? This shouldn't happen in normal usage
            panic!("Unexpected closure of the sender pool (channel) in SenderService")
        }
    }

    /// Sends a request to send a message which will be processed sequentially by the service (in refrence to other requests).
    /// 
    /// Parameters:
    /// * `msg` - The message to be sent
    /// 
    /// Returns: A boolean indicating if the message was sent successfully.
    pub async fn send(&self, msg: String) -> bool {
        let (reply_tx, reply_rx) = oneshot::channel();
        let send_msg_request = SenderServiceAction::SendMessage(SendMessage{ msg, reply: reply_tx});

        // Send request to the broker
        if self.actions_tx.send(send_msg_request).await.is_err() {
            // broker task has gone away
            debug!("Action request channel in SenderService has broken");
            return false;
        }

        // Wait for the broker to respond
        match reply_rx.await {
            Ok(success) => success,
            Err(_) => {
                debug!("Broker dropped the reply half?!");
                false
            }
        }
    }

    /// Creates a new pool of senders to use, replacing the current pool.
    /// 
    /// Parameters:
    /// * `new_senders` - The new senders to be used in the pool
    /// 
    /// Returns: A boolean indicating if the new pool was created and replaced the current pool successfully.
    pub async fn new_sender_pool(&self, new_senders: Vec<SimSender>) -> Result<(), mpsc::error::SendError<Self>> {
        let (reply_tx, _reply_rx) = oneshot::channel();
        let new_pool_request = SenderServiceAction::NewSenderPool(NewSenderPoolRequest{ new_senders, reply: reply_tx });
        // Send a message to the broker to shut down
        self.actions_tx.send(new_pool_request).await.expect("Failed to send shut down message to broker");

        Ok(())
    }

    /// Returns a the shared progress monitor used by the service.
    pub fn get_progress_monitor(&self) -> Arc<ProgressMonitor> {
        self.pm.clone()
    }

    /// Shuts down the service, and waits for all actions to be processed.
    ///
    /// Returns: A boolean indicating if the service was shut down successfully.
    pub async fn shut_down(&self) -> bool {
        let (reply_tx, reply_rx) = oneshot::channel();
        let shut_down_request = SenderServiceAction::ShutDown(reply_tx);
        // Send a message to the handler in the new function to shut down
        self.actions_tx.send(shut_down_request).await.expect("Failed to send shut down message to broker");

        // Wait for the handling loop to respond
        match reply_rx.await {
            Ok(_) => true,
            Err(_) => {
                debug!("Broker dropped the reply half?!");
                false
            }
        }
    }
}