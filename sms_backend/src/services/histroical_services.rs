use sea_orm::{
    DatabaseConnection, EntityTrait, QueryFilter, ColumnTrait
};
use entity::alert::{Column as AlertColumn, Entity as AlertEntity, Model};
use crate::utils::db_utils::db_error_utils::Error;


pub async fn get_user_history(user_id: u64, db: DatabaseConnection) -> Result<Vec<Model>, Error> {
    let historical_alerts = AlertEntity::find()
        .filter(AlertColumn::UserId.eq(user_id))
        .all(&db)
        .await?;

    Ok(historical_alerts)
}

pub async fn get_all_alert_data(db: DatabaseConnection) -> Result<Vec<Model>, Error> {
    let historical_alerts = AlertEntity::find()
        .all(&db)
        .await?;

    Ok(historical_alerts)
}