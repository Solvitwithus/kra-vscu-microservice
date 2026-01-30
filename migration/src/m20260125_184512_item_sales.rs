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
                    .col(
    ColumnDef::new(Sales::Generated_invc_no)
        .big_integer() 
        .not_null()
        .default(Expr::value(Value::Int(Some(0)))),
)
                    .col(string(Sales::Status))
                    .col(timestamp(Sales::CreatedAt).default(Expr::current_timestamp()))
                    .col(timestamp_null(Sales::UpdatedAt))

                    // === INVOICE CORE ===
                    .col(string(Sales::Tin))
                    .col(string(Sales::BhfId))
                    .col(integer_null(Sales::TrdInvcNo))

                    .col(big_integer_null(Sales::InvcNo))
                    .col(big_integer_null(Sales::OrgInvcNo))

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
                    .col(integer_null(Sales::TotItemCnt))

                    .col(double_null(Sales::TaxblAmtA))
                    .col(double_null(Sales::TaxblAmtB))
                    .col(double_null(Sales::TaxblAmtC))
                    .col(double_null(Sales::TaxblAmtD))
                    .col(double_null(Sales::TaxblAmtE))

                    .col(double_null(Sales::TaxRtA))
                    .col(double_null(Sales::TaxRtB))
                    .col(double_null(Sales::TaxRtC))
                    .col(double_null(Sales::TaxRtD))
                    .col(double_null(Sales::TaxRtE))

                    .col(double_null(Sales::TaxAmtA))
                    .col(double_null(Sales::TaxAmtB))
                    .col(double_null(Sales::TaxAmtC))
                    .col(double_null(Sales::TaxAmtD))
                    .col(double_null(Sales::TaxAmtE))

                    .col(double_null(Sales::TotTaxblAmt))
                    .col(double_null(Sales::TotTaxAmt))
                    .col(double_null(Sales::TotAmt))
                       .col( ColumnDef::new(Sales::RetryCount)
        .big_integer() 
        .not_null()
        .default(Expr::value(Value::Int(Some(0)))),
)
                    .col(double_null(Sales::NextRetryAt))

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
                    .col(json_binary_null(Sales::Response))

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
    Generated_invc_no,
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
RetryCount,
NextRetryAt,
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
    Response,
}
