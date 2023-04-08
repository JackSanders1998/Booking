use sea_orm_migration::prelude::*;

pub struct Migration;

impl MigrationName for Migration {
    fn name(&self) -> &str {
        "m20230408_000001_create_venue_table"
    }
}

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Venue::Table)
                    .col(
                        ColumnDef::new(Venue::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Venue::Name).string().not_null())
                    .col(ColumnDef::new(Venue::Description).string().not_null())
                    .col(ColumnDef::new(Venue::Address).string().not_null())
                    .col(ColumnDef::new(Venue::Seats).integer())
                    .col(ColumnDef::new(Venue::Published).boolean().default(false))
                    .col(ColumnDef::new(Venue::CreatedAt).timestamp())
                    .col(ColumnDef::new(Venue::LastModified).timestamp())
                    .to_owned(),
            )
            .await
    }

    // Define how to rollback this migration: Drop the Venue table.
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Venue::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
pub enum Venue {
    Table,
    Id,
    Name,
    Description,
    Address,
    Seats,
    Published,
    CreatedAt,
    LastModified,
}
