use axum::{routing::get, Router};
use sea_orm::DatabaseConnection;

use crate::handlers;


/// Creates a router with sub routes for historical data related operations, using the database connection.
/// 
/// supported routes:
/// *  /my: GET - Fetch user's own historical data (WARNING / TODO: currently hardcoded to user 1)
/// *  /all: GET - Fetch all historical data 
pub fn historical_routes() -> Router<DatabaseConnection> {
    Router::new()
        .route("/my", get(handlers::historical_handler::get_my_history)) // Fetch user's own historical data
        .route("/all", get(handlers::historical_handler::get_all_history)) // Fetch all historical data
}