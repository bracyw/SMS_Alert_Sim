use std::sync::Arc;
use axum::extract::State;
use axum::{Extension, Json};
use entity::alert::Entity as AlertEntity;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::Deserialize;
use sea_orm::ActiveValue;
use tracing_subscriber::field::debug;
use crate::models::alert_models::Alert;
use crate::services::alert_services;
use crate::structures::sender_service::SenderService;
use tracing::debug;


/// Parameters for sending out an alert
/// * msg - The message to send (if this is None then the Alert will send random messages)
/// * num_msgs - The number of messages to send
#[derive(Deserialize)]
pub struct SendMsgParams {
    msg: Option<String>,
    num_msgs: f64,
}


/// Send the given msg to the given number of senders each with the given failure percent, average send time, and standard deviation provided in the path.
/// Will add the alert info returned to the database, for historical tracking.
/// 
/// Parameters:
/// 
/// * db - The database connection to use to insert the alert outcome into the database
/// * sender_service - The sender service to use to send the messages
/// * params - The parameters for the alert (see SendMsgParams for details)
/// 
/// Returns: a json response with the stats as an `Alert` client model (in the models dir) or an error message.
#[axum::debug_handler]
pub async fn send_alert(
    State(db): State<DatabaseConnection>,
    Extension(sender_service): Extension<Arc<SenderService>>,
    Json(params): Json<SendMsgParams>,
) -> Result<Json<Alert>, axum::response::Response> {
    debug!("Sending alert with message: {}", params.msg.clone().unwrap_or_else(|| "No message provided sending random".to_owned()));
    
    // Send the messages and get the stats back from that alert
    let send_stats = alert_services::send_msgs(params.msg.clone(), params.num_msgs as u64, sender_service).await;

    // Convert to a database entity (could be abstracted if there is larger use of models in the future)
    let alert_entity = entity::alert::ActiveModel {
        user_id: ActiveValue::Set(1),
        send_amount_requested: ActiveValue::Set(params.num_msgs as i32),
        message: ActiveValue::Set(params.msg),
        messages_sent: ActiveValue::Set(send_stats.messages_sent as i32),
        messages_failed: ActiveValue::Set(send_stats.messages_failed as i32),
        total_time_to_send: ActiveValue::Set(send_stats.total_time_to_send as i32),
        ..Default::default()
    };
    debug!("Alert entity created: {:?}", alert_entity);


    // Insert the alert into the database
    AlertEntity::insert(alert_entity)
        .exec(&db)
        .await
        .expect("Failed to insert alert into database");

    
    debug!("Alert sent with stats, messages sent: {}, messages failed: {}, total time to send: {}", send_stats.messages_sent, send_stats.messages_failed, send_stats.total_time_to_send);
    Ok(Json::from(send_stats))
}









