
/// Representation of a past alert for the client
/// snake case must be used in clients to match the json response (simply easier to read)
#[derive(serde::Serialize)]
pub struct Alert {
    pub send_amount_requested: i32,
    pub messages_sent: i32,
    pub messages_failed: i32,
    pub total_time_to_send: i32,
}



impl From<entity::alert::Model> for Alert {
    fn from(alert_entity: entity::alert::Model) -> Alert {
        Alert {
            send_amount_requested: alert_entity.send_amount_requested,
            messages_sent: alert_entity.messages_sent,
            messages_failed: alert_entity.messages_failed,
            total_time_to_send: alert_entity.total_time_to_send,
        }
    }
}