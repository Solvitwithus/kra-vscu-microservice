use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ItemMaster::Table)
                    .modify_column(
                        ColumnDef::new(ItemMaster::BtchNo)
                            .string()
                            .null()
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(ItemMaster::Table)
                    .modify_column(
                        ColumnDef::new(ItemMaster::BtchNo)
                            .string()
                            .not_null()
                    )
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ItemMaster {
    Table,
    BtchNo,
}
