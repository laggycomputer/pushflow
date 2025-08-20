pub use sea_orm_migration::prelude::*;

mod m20250808_012056_create_model;
mod m20250820_064413_subscriber_keys_separate;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250808_012056_create_model::Migration),
            Box::new(m20250820_064413_subscriber_keys_separate::Migration),
        ]
    }
}
