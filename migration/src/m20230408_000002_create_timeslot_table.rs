use sea_orm_migration::prelude::*;

use super::m20230408_000001_create_venue_table::Venue;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230408_000002_create_timeslot_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Timeslot::Table)
                    .col(
                        ColumnDef::new(Timeslot::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Timeslot::Start).date_time().not_null())
                    .col(ColumnDef::new(Timeslot::End).date_time().not_null())
                    .col(ColumnDef::new(Timeslot::VenueId).integer().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk-timeslot-venue_id")
                            .from(Timeslot::Table, Timeslot::VenueId)
                            .to(Venue::Table, Venue::Id),
                    )
                    .col(ColumnDef::new(Timeslot::CreatedAt).timestamp())
                    .col(ColumnDef::new(Timeslot::LastModified).timestamp())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Timeslot table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Timeslot::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Timeslot {
    Table,
    Id,
    Start,
    End,
    VenueId,
    CreatedAt,
    LastModified,
}
