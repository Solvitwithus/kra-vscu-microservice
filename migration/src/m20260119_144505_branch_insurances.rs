use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the branch insurance table
        manager
            .create_table(
                Table::create()
                    .table(BhfInsurance::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BhfInsurance::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BhfInsurance::Tin).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::BhfId).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::IsrccCd).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::IsrccNm).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::IsrcRt).unsigned().not_null())
                    .col(ColumnDef::new(BhfInsurance::UseYn).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::RegrNm).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::RegrId).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::ModrNm).string().not_null())
                    .col(ColumnDef::new(BhfInsurance::ModrId).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop the branch insurance table
        manager
            .drop_table(Table::drop().table(BhfInsurance::Table).to_owned())
            .await
    }
}

// Table column identifiers
#[derive(DeriveIden)]
enum BhfInsurance {
    Table,
    Id,
    Tin,
    BhfId,
    IsrccCd,
    IsrccNm,
    IsrcRt,
    UseYn,
    RegrNm,
    RegrId,
    ModrNm,
    ModrId,
}
