use std::sync::Arc;

use axum::{routing::post, Extension, Router};
use sea_orm::DatabaseConnection;

use crate::{handlers, structures::sender_service::SenderService};


/// Creates a router with sub routes for system related operations, currently sender service management.
/// 
/// Parameters:
/// * `sender_service` - The sender service to use in the routes
/// 
/// supported routes:
/// *  /sender-pool/new: POST - Create a new sender service
/// 
/// Returns: A router with the above routes
pub fn system_routes(sender_service: Arc<SenderService>) -> Router<DatabaseConnection> {
    Router::new()
    .route("/sender-pool/new", post(handlers::system_handler::change_sender_pool)) // Create a new sender service

    // ROUTES THAT CAN BE ADDED LATER
    // .route("/info", get(handler::sender_service_handler::get_sender_service_info)) // Get sender service info
    // .route("/shut-down", post(handlers::sender_service_handler::shut_down)) // Stop all sending activities
    
    .layer(Extension(sender_service.clone()))
}