use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Sales::Table)
                    .if_not_exists()
                    .col(pk_auto(Sales::Id))

                    // === AUTH / META ===
                    .col(string(Sales::ApiKey))
                    .col(string(Sales::Status))
                    .col(timestamp(Sales::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Sales::UpdatedAt))

                    // === INVOICE CORE ===
                    .col(string(Sales::Tin))
                    .col(string(Sales::BhfId))
                    .col(big_integer(Sales::TrdInvcNo))

                    .col(big_integer(Sales::InvcNo))
                    .col(big_integer(Sales::OrgInvcNo))

                    .col(string(Sales::CustTin))
                    .col(string(Sales::CustNm))

                    .col(string(Sales::SalesTyCd))
                    .col(string(Sales::RcptTyCd))
                    .col(string(Sales::PmtTyCd))
                    .col(string(Sales::SalesSttsCd))

                    .col(string(Sales::CfmDt))
                    .col(string(Sales::SalesDt))
                    .col(string(Sales::StockRlsDt))

                    .col(string_null(Sales::CnclReqDt))
                    .col(string_null(Sales::CnclDt))
                    .col(string_null(Sales::RfdDt))
                    .col(string_null(Sales::RfdRsnCd))

                    // === TOTALS ===
                    .col(integer(Sales::TotItemCnt))

                    .col(double(Sales::TaxblAmtA))
                    .col(double(Sales::TaxblAmtB))
                    .col(double(Sales::TaxblAmtC))
                    .col(double(Sales::TaxblAmtD))
                    .col(double(Sales::TaxblAmtE))

                    .col(double(Sales::TaxRtA))
                    .col(double(Sales::TaxRtB))
                    .col(double(Sales::TaxRtC))
                    .col(double(Sales::TaxRtD))
                    .col(double(Sales::TaxRtE))

                    .col(double(Sales::TaxAmtA))
                    .col(double(Sales::TaxAmtB))
                    .col(double(Sales::TaxAmtC))
                    .col(double(Sales::TaxAmtD))
                    .col(double(Sales::TaxAmtE))

                    .col(double(Sales::TotTaxblAmt))
                    .col(double(Sales::TotTaxAmt))
                    .col(double(Sales::TotAmt))

                    // === FLAGS ===
                    .col(string(Sales::PrchrAcptcYn))
                    .col(string_null(Sales::Remark))

                    .col(string(Sales::RegrId))
                    .col(string(Sales::RegrNm))
                    .col(string(Sales::ModrId))
                    .col(string(Sales::ModrNm))

                    // === NESTED JSON ===
                    .col(json_binary(Sales::Receipt))
                    .col(json_binary(Sales::ItemList))

                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Sales::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sales {
    Table,
    Id,

    // meta
    ApiKey,
    Status,
    CreatedAt,
    UpdatedAt,

    // invoice
    Tin,
    BhfId,
    TrdInvcNo,
    InvcNo,
    OrgInvcNo,

    CustTin,
    CustNm,

    SalesTyCd,
    RcptTyCd,
    PmtTyCd,
    SalesSttsCd,

    CfmDt,
    SalesDt,
    StockRlsDt,

    CnclReqDt,
    CnclDt,
    RfdDt,
    RfdRsnCd,

    TotItemCnt,

    TaxblAmtA,
    TaxblAmtB,
    TaxblAmtC,
    TaxblAmtD,
    TaxblAmtE,

    TaxRtA,
    TaxRtB,
    TaxRtC,
    TaxRtD,
    TaxRtE,

    TaxAmtA,
    TaxAmtB,
    TaxAmtC,
    TaxAmtD,
    TaxAmtE,

    TotTaxblAmt,
    TotTaxAmt,
    TotAmt,

    PrchrAcptcYn,
    Remark,

    RegrId,
    RegrNm,
    ModrId,
    ModrNm,

    Receipt,
    ItemList,
}
