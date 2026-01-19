use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the bhf_users table
        manager
            .create_table(
                Table::create()
                    .table(BhfUsers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(BhfUsers::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(BhfUsers::Tin).string_len(11).not_null())
                    .col(ColumnDef::new(BhfUsers::BhfId).string_len(2).not_null())
                    .col(ColumnDef::new(BhfUsers::UserId).string_len(20).not_null())
                    .col(ColumnDef::new(BhfUsers::UserNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfUsers::Pwd).string_len(255).not_null())
                    .col(ColumnDef::new(BhfUsers::Adrs).string_len(200).null())
                    .col(ColumnDef::new(BhfUsers::Cntc).string_len(20).null())
                    .col(ColumnDef::new(BhfUsers::AuthCd).string_len(100).null())
                    .col(ColumnDef::new(BhfUsers::Remark).string_len(2000).null())
                    .col(ColumnDef::new(BhfUsers::UseYn).string_len(1).not_null())
                    .col(ColumnDef::new(BhfUsers::RegrNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfUsers::RegrId).string_len(20).not_null())
                    .col(ColumnDef::new(BhfUsers::ModrNm).string_len(60).not_null())
                    .col(ColumnDef::new(BhfUsers::ModrId).string_len(20).not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(BhfUsers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum BhfUsers {
    Table,
    Id,
    Tin,
    BhfId,
    UserId,
    UserNm,
    Pwd,
    Adrs,
    Cntc,
    AuthCd,
    Remark,
    UseYn,
    RegrNm,
    RegrId,
    ModrNm,
    ModrId,
}
