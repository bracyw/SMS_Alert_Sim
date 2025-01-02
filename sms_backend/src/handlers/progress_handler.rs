use std::sync::Arc;
use axum::{Extension, Json};
use crate::{models::progress_models::SendProgressData, services, structures::progress_monitor::ProgressMonitor};
use services::progress_services;


#[axum_macros::debug_handler]
/// Get's all the current progress data points from the progress monitor
/// 
/// Parameters:
/// * `pm` - The progress monitor to get the data from
/// 
/// Returns: A json object with all the progress data points (see SendProgressData for details) or an Erorr response
pub async fn all_progress_data(
    Extension(pm): Extension<Arc<ProgressMonitor>>,
) -> Result<Json<SendProgressData>, axum::response::Response>  {
    Ok(Json::from(progress_services::get_progress_data(pm)))
}

#[axum_macros::debug_handler]
/// Get the current number of messages sent
/// 
/// Parameters:
/// * `pm` - The progress monitor to get the data from
/// 
/// Returns: A json object with the number of messages sent or an Erorr response
pub async fn get_msgs_sent(
    Extension(pm): Extension<Arc<ProgressMonitor>>
) -> Result<Json<u64>, axum::response::Response> {
    Ok(Json(pm.get_msgs_sent()))
}

#[axum_macros::debug_handler]
/// Get the current number of messages failed
/// 
/// Parameters:
/// * `pm` - The progress monitor to get the data from
/// 
/// Returns: A json object with the number of messages failed or an Erorr response
pub async fn get_msgs_failed(
    Extension(pm): Extension<Arc<ProgressMonitor>>,
) -> Result<Json<u64>, axum::response::Response> {
    Ok(Json(pm.get_msgs_failed()))
}

#[axum_macros::debug_handler]
/// Get the current average wait time
/// 
/// Parameters:
/// * `pm` - The progress monitor to get the data from
/// 
/// Returns: A json object with the average wait time or an Erorr response
pub async fn get_avg_wait_time(
    Extension(pm): Extension<Arc<ProgressMonitor>>,
) -> Result<Json<u64>, axum::response::Response> {
    Ok(Json(pm.get_avg_wait_time()))
}


#[axum_macros::debug_handler]
/// Reset the progress monitor statistics
/// 
/// Parameters:
/// * `pm` - The progress monitor to reset
/// 
/// Returns: A json object with a boolean value of true or an Erorr response
pub async fn reset_progress_monitor(
    Extension(pm): Extension<Arc<ProgressMonitor>>,
) -> Result<Json<bool>, axum::response::Response> {
    pm.reset();
    Ok(Json(true))
}
