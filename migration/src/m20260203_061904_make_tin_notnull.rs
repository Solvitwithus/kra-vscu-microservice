use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
     

        manager
            .create_table(
                Table::create()
                    .table(StockMaster::Table)
                    .if_not_exists()
                    .col(pk_auto(StockMaster::Id))
                    .col(string(StockMaster::Tin))
                    .col(string(StockMaster::BhfId))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
       

        manager
            .drop_table(Table::drop().table(StockMaster::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum StockMaster {
    Table,
    Id,
     Tin,
    BhfId,
}
