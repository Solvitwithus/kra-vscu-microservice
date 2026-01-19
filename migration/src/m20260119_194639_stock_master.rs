use sea_orm_migration::prelude::*;

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
                    .col(ColumnDef::new(StockMaster::Id).big_integer().auto_increment().primary_key())
                    .col(ColumnDef::new(StockMaster::Tin).char().string_len(11).not_null())
                    .col(ColumnDef::new(StockMaster::BhfId).char().string_len(2).not_null())
                    .col(ColumnDef::new(StockMaster::ItemCd).string().string_len(20).not_null())
                    .col(ColumnDef::new(StockMaster::RsdQty).decimal_len(13, 2).not_null())
                    .col(ColumnDef::new(StockMaster::RegrNm).string().string_len(60).not_null())
                    .col(ColumnDef::new(StockMaster::RegrId).string().string_len(20).not_null())
                    .col(ColumnDef::new(StockMaster::ModrNm).string().string_len(60).not_null())
                    .col(ColumnDef::new(StockMaster::ModrId).string().string_len(20).not_null())
                    .col(timestamp(StockMaster::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp(StockMaster::UpdatedAt).default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_stock_master_unique")
                    .table(StockMaster::Table)
                    .col(StockMaster::Tin)
                    .col(StockMaster::BhfId)
                    .col(StockMaster::ItemCd)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager.drop_index(Index::drop().name("idx_stock_master_unique").table(StockMaster::Table).to_owned()).await?;
        manager.drop_table(Table::drop().table(StockMaster::Table).to_owned()).await
    }
}

#[derive(DeriveIden)]
enum StockMaster {
    Table,
    Id,
    Tin,
    BhfId,
    ItemCd,
    RsdQty,
    RegrNm,
    RegrId,
    ModrNm,
    ModrId,
    CreatedAt,
    UpdatedAt,
}
