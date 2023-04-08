pub use sea_orm_migration::prelude::*;

mod m20230408_000001_create_venue_table;
mod m20230408_000002_create_timeslot_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            // Define the order of migrations.
            Box::new(m20230408_000001_create_venue_table::Migration),
            Box::new(m20230408_000002_create_timeslot_table::Migration),
        ]
    }
}
