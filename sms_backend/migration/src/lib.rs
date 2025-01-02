pub mod m20230101_create_user;
pub mod m20230102_create_alert;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20230101_create_user::Migration),
            Box::new(m20230102_create_alert::Migration),
        ]
    }
}
