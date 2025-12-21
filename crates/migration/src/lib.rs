pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20251220_000001_create_user_table;
mod m20251221_110556_add_deleted_at_to_user;
mod m20251221_144201_split_user_name_and_add_birth_date;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20251220_000001_create_user_table::Migration),
            Box::new(m20251221_110556_add_deleted_at_to_user::Migration),
            Box::new(m20251221_144201_split_user_name_and_add_birth_date::Migration),
        ]
    }
}
