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
                    .col(string(ItemMaster::Tin))
                    .col(string(ItemMaster::BhfId))
                    .col(string(ItemMaster::Status))
                    .col(string(ItemMaster::ItemCd).not_null())
                    .col(string(ItemMaster::ItemClsCd).not_null())
                    .col(string(ItemMaster::ItemTyCd).not_null())
                    .col(string(ItemMaster::ItemNm).not_null())

                    // Optional
                    .col(string(ItemMaster::ItemStdNm))

                    // Classification
                    .col(string(ItemMaster::OrgnNatCd).not_null())
                    .col(string(ItemMaster::PkgUnitCd).not_null())
                    .col(string(ItemMaster::QtyUnitCd).not_null())
                    .col(string(ItemMaster::TaxTyCd).not_null())

                    // Optional identifiers
                    .col(string(ItemMaster::BtchNo))
                    .col(string(ItemMaster::Bcd))

                    // Prices
                    .col(decimal_len(ItemMaster::DftPrc, 18, 2).not_null())
                    .col(decimal_len(ItemMaster::GrpPrcL1, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL2, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL3, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL4, 18, 2))
                    .col(decimal_len(ItemMaster::GrpPrcL5, 18, 2))

                    // Other
                    .col(string(ItemMaster::AddInfo))
                    .col(decimal_len(ItemMaster::SftyQty, 13, 2))
                    .col(string(ItemMaster::IsrcAplcbYn).not_null())
                    .col(string(ItemMaster::UseYn).not_null())

                    // Audit
                    .col(string(ItemMaster::RegrNm).not_null())
                    .col(string(ItemMaster::RegrId).not_null())
                    .col(string(ItemMaster::ModrNm).not_null())
                    .col(string(ItemMaster::ModrId).not_null())
                    .col(json_binary_null(ItemMaster::Response))

                    // Unique constraint
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
    Response,
}
