mod m20250709_162257_create_articles_table;
mod m20260318_095749_add_tags;

pub use sea_orm_migration::prelude::*;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250709_162257_create_articles_table::Migration),
            Box::new(m20260318_095749_add_tags::Migration),
        ]
    }
}
