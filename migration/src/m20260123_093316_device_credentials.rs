use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {

        manager
            .create_table(
                Table::create()
                    .table(Credentials::Table)
                    .if_not_exists()
                    .col(pk_auto(Credentials::Id))

                    .col(string(Credentials::CompanyId).not_null())
                    .col(string(Credentials::EnvironmentName).not_null())
                    .col(string(Credentials::EnvironmentUrl).not_null())
                    .col(string(Credentials::Pin).not_null())
                    .col(string(Credentials::BranchId).not_null())

                    .col(string(Credentials::DeviceSerial).not_null())
                    .col(string(Credentials::ApiKey).not_null())

                    /* -------- UNIQUE INDEXES -------- */
                    .index(
                        Index::create()
                            .unique()
                            .name("uniq_device_serial")
                            .col(Credentials::DeviceSerial)
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("uniq_api_key")
                            .col(Credentials::ApiKey)
                    )
                    .index(
                        Index::create()
                            .unique()
                            .name("uniq_pin_per_env")
                            .col(Credentials::Pin)
                            .col(Credentials::EnvironmentName)
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Credentials::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Credentials {
    Table,
    Id,
    CompanyId,
    EnvironmentName,
    EnvironmentUrl,
    Pin,
    BranchId,
    DeviceSerial,
    ApiKey,
}
