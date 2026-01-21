use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()

                    // Primary key
                    .col(pk_auto(User::Id))

                    // User info
                    .col(string(User::FullName))
                    .col(string(User::Email).unique_key())
                    .col(string(User::PhoneNumber))

                    // Security
                    .col(string(User::PasswordHash))

                    // Terms agreement
                    .col(boolean(User::Agreement).default(false))

                    // Timestamps
                    .col(ColumnDef::new(User::CreatedAt).timestamp().default(Expr::current_timestamp()))
                    .col(ColumnDef::new(User::UpdatedAt).timestamp().default(Expr::current_timestamp()))
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum User {
    Table,
    Id,
    FullName,
    Email,
    PhoneNumber,
    PasswordHash,
    Agreement,
    CreatedAt,
    UpdatedAt,
}
