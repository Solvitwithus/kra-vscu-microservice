use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(BhfCustomer::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BhfCustomer::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    
                    .col(ColumnDef::new(BhfCustomer::Tin).string_len(11).not_null())
                    .col(ColumnDef::new(BhfCustomer::BhfId).string_len(2).not_null())
                    .col(ColumnDef::new(BhfCustomer::CustNo).string_len(9).not_null())
                    .col(ColumnDef::new(BhfCustomer::CustTin).string_len(11).not_null())
                    .col(ColumnDef::new(BhfCustomer::CustNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfCustomer::Adrs).string_len(300).null())
                    .col(ColumnDef::new(BhfCustomer::TelNo).string_len(20).null())
                    .col(ColumnDef::new(BhfCustomer::Email).string_len(50).null())
                    .col(ColumnDef::new(BhfCustomer::FaxNo).string_len(20).null())
                    .col(ColumnDef::new(BhfCustomer::UseYn).string_len(1).not_null())
                    .col(ColumnDef::new(BhfCustomer::Remark).string_len(1000).null())
                    .col(ColumnDef::new(BhfCustomer::RegrNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfCustomer::RegrId).string_len(20).not_null())
                    .col(ColumnDef::new(BhfCustomer::ModrNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfCustomer::ModrId).string_len(20).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BhfCustomer::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BhfCustomer {
    Table,
    Id,
    Tin,
    BhfId,
    CustNo,
    CustTin,
    CustNm,
    Adrs,
    TelNo,
    Email,
    FaxNo,
    UseYn,
    Remark,
    RegrNm,
    RegrId,
    ModrNm,
    ModrId,
}
