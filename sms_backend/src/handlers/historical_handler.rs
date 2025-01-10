use axum::{
    extract::State, Json
};
use sea_orm::DatabaseConnection;
use crate::{models::alert_models::Alert, services::histroical_services::{get_all_alert_data, get_user_history}, utils::db_utils::db_error_utils::Error};  


/// TODO: Currently this function is hardcoded to return the alerts for user 1. Replace this with actual user authentication logic for production.
/// Get the historical alerts sent by the current user. 
/// * db - The database connection
#[axum_macros::debug_handler]
pub async fn get_my_history(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Alert>>, Error> {
    let user_id = 1; // TODO: Replace with actual user authentication logic.

    get_user_history(user_id, db).await
        .map(|historical_alerts| Json(
            historical_alerts
                .into_iter()
                .rev()
                .map(Alert::from) // Simplified function call for mapping.
                .collect(),
        ))
}


/// Get all historical alerts sent by all users.
/// * db - The database connection
#[axum_macros::debug_handler]
pub async fn get_all_history(
    State(db): State<DatabaseConnection>,
) -> Result<Json<Vec<Alert>>, Error> {
    // TODO: where user validation logic would go

    get_all_alert_data(db).await
        .map(|historical_alerts| Json(
            historical_alerts
                .into_iter().rev()
                .map(Alert::from) // Simplified function call for mapping.
                .collect(),
        ))
}
