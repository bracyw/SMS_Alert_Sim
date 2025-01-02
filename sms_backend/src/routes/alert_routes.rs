use std::sync::Arc;

use axum::{routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::{handlers, structures::sender_service::SenderService};


/// Creates a router with sub routes for alert related operations
/// 
/// Parameters:
/// * `sender_service` - The sender service to use in the routes
/// 
/// supported routes:
/// *  /send: POST - Sends an alert with a certain amount of messages
pub fn alert_routes(sender_service: Arc<SenderService>) -> Router<DatabaseConnection> {
    Router::new()
    .route("/send", post(handlers::alert_handler::send_alert)) // Send an alert
    .layer(Extension(sender_service.clone()))
}