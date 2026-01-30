use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(ItemMaster::Table)
                    .if_not_exists()
                    .col(pk_auto(ItemMaster::Id))

                    // Required
                    .col(string_len(ItemMaster::Tin, 11))
                    .col(string_len(ItemMaster::BhfId, 2))
                    .col(string(ItemMaster::Status))
                    .col(string_len(ItemMaster::ItemCd, 20).not_null())
                    .col(string_len(ItemMaster::ItemClsCd, 10).not_null())
                    .col(string_len(ItemMaster::ItemTyCd, 5).not_null())
                    .col(string_len(ItemMaster::ItemNm, 200).not_null())

                    // Optional
                    .col(string_len(ItemMaster::ItemStdNm, 200))

                    // Classification
                    .col(string_len(ItemMaster::OrgnNatCd, 5).not_null())
                    .col(string_len(ItemMaster::PkgUnitCd, 5).not_null())
                    .col(string_len(ItemMaster::QtyUnitCd, 5).not_null())
                    .col(string_len(ItemMaster::TaxTyCd, 5).not_null())

                    // Optional identifiers
                    .col(string_len(ItemMaster::BtchNo, 10))
                    .col(string_len(ItemMaster::Bcd, 20))

                    // Prices
                    .col(decimal_len(ItemMaster::DftPrc, 18, 2).not_null())
                    .col(decimal_len(ItemMaster::GrpPrcL1, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL2, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL3, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL4, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL5, 18, 2))

                    // Other
                    .col(string_len(ItemMaster::AddInfo, 7))
                    .col(decimal_len(ItemMaster::SftyQty, 13, 2))
                    .col(string_len(ItemMaster::IsrcAplcbYn, 1).not_null())
                    .col(string_len(ItemMaster::UseYn, 1).not_null())

                    // Audit
                    .col(string_len(ItemMaster::RegrNm, 60).not_null())
                    .col(string_len(ItemMaster::RegrId, 20).not_null())
                    .col(string_len(ItemMaster::ModrNm, 60).not_null())
                    .col(string_len(ItemMaster::ModrId, 20).not_null())

                    // Unique constraint (VERY IMPORTANT)
                    .index(
                        Index::create()
                            .name("uq_item_master_tin_bhf_item")
                            .col(ItemMaster::Tin)
                            .col(ItemMaster::BhfId)
                            .col(ItemMaster::ItemCd)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(ItemMaster::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum ItemMaster {
    Table,
    Id,
    Tin,
    BhfId,
    Status,
    ItemCd,
    ItemClsCd,
    ItemTyCd,
    ItemNm,
    ItemStdNm,
    OrgnNatCd,
    PkgUnitCd,
    QtyUnitCd,
    TaxTyCd,
    BtchNo,
    Bcd,
    DftPrc,
    GrpPrcL1,
    GrpPrcL2,
    GrpPrcL3,
    GrpPrcL4,
    GrpPrcL5,
    AddInfo,
    SftyQty,
    IsrcAplcbYn,
    UseYn,
    RegrNm,
    RegrId,
    ModrNm,
    ModrId,
}
