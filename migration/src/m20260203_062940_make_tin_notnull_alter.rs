use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockMaster::Table)
                    .add_column(ColumnDef::new(StockMaster::Tin).string_len(11).not_null())
                    .add_column(ColumnDef::new(StockMaster::BhfId).string_len(2).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(StockMaster::Table)
                    .drop_column(StockMaster::Tin)
                    .drop_column(StockMaster::BhfId)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum StockMaster {
    Table,
    Tin,
    BhfId,
}
