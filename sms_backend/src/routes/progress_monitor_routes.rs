use std::sync::Arc;
use axum::{routing::{get, post}, Extension, Router};
use sea_orm::DatabaseConnection;
use crate::{handlers, structures::progress_monitor::ProgressMonitor};


/// Creates a router with sub routes for progress monitor related operations. 
/// Not to be confused with historical time data, these routes return data about current system run health and performance.
/// (for more info see `ProgressMonitor` in `progress_monitor.rs`)
/// 
/// Parameters:
/// * `pm` - The progress monitor to use in the routes
/// 
/// supported routes:
/// *  /all-info: GET - Get all progress data points (messages sent, messages failed, average wait time)
/// *  /sent: GET - Get the number of messages sent
/// *  /failed: GET - Get the number of messages failed
/// *  /avg-wait: GET - Get the average wait time
/// *  /reset: POST - Reset the progress monitor
/// 
/// Returns: A router with the above routes
pub fn progress_monitor_routes(pm: Arc<ProgressMonitor>) -> Router<DatabaseConnection> {
    Router::new()
        .route("/all-info", get(handlers::progress_handler::all_progress_data)) // All progress data
        .route("/sent", get(handlers::progress_handler::get_msgs_sent)) // Messages sent
        .route("/failed", get(handlers::progress_handler::get_msgs_failed)) // Messages failed
        .route("/avg-wait", get(handlers::progress_handler::get_avg_wait_time)) // Average wait time
        .route("/reset", post(handlers::progress_handler::reset_progress_monitor)) // Reset progress monitor
        .layer(Extension(pm.clone()))
}