use sea_orm_migration::prelude::*;
use crate::m20230101_create_user::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Alert::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Alert::Id).integer().not_null().primary_key().auto_increment())
                    .col(ColumnDef::new(Alert::UserId).integer().not_null())
                    .col(ColumnDef::new(Alert::SendAmountRequested).integer().not_null())
                    .col(ColumnDef::new(Alert::Message).string().null())
                    .col(ColumnDef::new(Alert::MessagesSent).integer().not_null().default(0))
                    .col(ColumnDef::new(Alert::MessagesFailed).integer().not_null().default(0))
                    .col(ColumnDef::new(Alert::TotalTimeToSend).integer().not_null().default(0))
                    .foreign_key(
                        ForeignKey::create()
                            .from(Alert::Table, Alert::UserId)
                            .to(User::Table, User::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_table(Table::drop().table(Alert::Table).to_owned()).await
    }
}

#[derive(Iden)]
pub enum Alert {
    Table,
    Id,
    UserId,
    SendAmountRequested,
    Message,
    MessagesSent,
    MessagesFailed,
    TotalTimeToSend,
}
