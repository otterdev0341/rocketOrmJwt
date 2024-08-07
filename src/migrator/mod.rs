pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_user_table;
mod m20240807_184242_create_author_table;
mod m20240807_184401_create_book_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_user_table::Migration),
            Box::new(m20240807_184242_create_author_table::Migration),
            Box::new(m20240807_184401_create_book_table::Migration),
        ]
    }
}
