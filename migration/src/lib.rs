pub use sea_orm_migration::prelude::*;



mod m20260119_134024_branch_customers;
mod m20260119_142033_branch_users;
mod m20260119_144505_branch_insurances;


pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260119_134024_branch_customers::Migration),
            Box::new(m20260119_142033_branch_users::Migration),
            Box::new(m20260119_144505_branch_insurances::Migration),
        ]
    }
}
