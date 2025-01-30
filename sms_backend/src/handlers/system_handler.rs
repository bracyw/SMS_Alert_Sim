use std::sync::Arc;

use axum::{http::StatusCode, response::IntoResponse, Extension, Json};
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

use crate::{structures::sender_service::SenderService, services::alert_services};

/// Paramater for creating a new sender pool
#[derive(Deserialize)]
pub struct NewSenderPoolParams {
    pub num_senders: f64,
    pub failure_percent: f64,
    pub avg_send_time: f64,
}

#[axum::debug_handler]
/// Changes the sender pool used by the shared SenderService to a new pool of senders with the given attributes.
/// 
/// Parameters:
/// * `sender_service` - The sender service to replace the sender pool of
/// * `params` - The parameters for the new sender pool (see `NewSenderPoolParams`)
/// 
/// Returns: A JSON object with a boolean value indicating if the sender pool was successfully changed
pub async fn change_sender_pool(
    Extension(sender_service): Extension<Arc<SenderService>>,
    Json(params): Json<NewSenderPoolParams>,
) -> Result<Json<bool>, axum::response::Response> {
    debug!("Creating sender pool with attributes: num_senders: {}, failure_percent: {}, avg_send_time: {}", params.num_senders, params.failure_percent, params.avg_send_time);
   
    // Example creation of a new service (adjust parameters as needed)
    let posible_senders = alert_services::create_sim_senders(
        params.num_senders,
        params.failure_percent,
        params.avg_send_time as f32,
    );
   

    match posible_senders {
        Ok(senders) => {
            // replace the sender pool with our new senders
            let _ = sender_service.new_sender_pool(senders).await;
            
            Ok(Json(true))
        },
        Err(e) => {
            let error_message = format!("Failed to create sender service: {}", e.to_string());
            let response_body = Json(json!({ "error": error_message }));
            Err((StatusCode::BAD_REQUEST, response_body).into_response())
        }
    }
}