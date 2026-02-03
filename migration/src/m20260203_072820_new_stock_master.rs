use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
      

        manager
            .create_table(
                Table::create()
                    .table(NewStockMaster::Table)
                    .if_not_exists()
                    .col(pk_auto(NewStockMaster::Id))
                    .col(string(NewStockMaster::Status))
                    .col(json_binary_null(NewStockMaster::Payload))
                    .col(json_binary_null(NewStockMaster::Response))
                    .col(integer(NewStockMaster::RetryCount))
                    .col(string(NewStockMaster::NextRetryAt))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
      

        manager
            .drop_table(Table::drop().table(NewStockMaster::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum NewStockMaster {
    Table,
    Id,
    Status,
    Payload,
    Response,
    RetryCount,
    NextRetryAt,

}
