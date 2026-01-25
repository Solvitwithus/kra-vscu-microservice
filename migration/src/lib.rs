pub use sea_orm_migration::prelude::*;



mod m20260119_134024_branch_customers;
mod m20260119_142033_branch_users;
mod m20260119_144505_branch_insurances;
mod m20260119_194639_stock_master;

mod m20260120_133135_product_save_items;
mod m20260121_190839_sign_up;

mod m20260123_093316_device_credentials;
mod m20260125_184512_item_sales;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260119_134024_branch_customers::Migration),
            Box::new(m20260119_142033_branch_users::Migration),
            Box::new(m20260119_144505_branch_insurances::Migration),
            Box::new(m20260119_194639_stock_master::Migration),
            Box::new(m20260120_133135_product_save_items::Migration),
            Box::new(m20260121_190839_sign_up::Migration),
            Box::new(m20260123_093316_device_credentials::Migration),
            Box::new(m20260125_184512_item_sales::Migration),
        ]
    }
}
