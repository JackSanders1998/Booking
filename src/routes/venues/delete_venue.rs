use crate::queries::venue_queries::{find_venue_by_id, save_active_venue};
use crate::{utilities::app_error::AppError};
use axum::{
    extract::{Path, State},
    Extension,
};
use chrono::Utc;
use sea_orm::{DatabaseConnection, IntoActiveModel, Set};

pub async fn soft_delete_venue(
    State(db): State<DatabaseConnection>,
    Path(venue_id): Path<i32>,
) -> Result<(), AppError> {
    let mut venue = find_venue_by_id(&db, venue_id)
        .await?
        .into_active_model();

    let now = Utc::now();

    venue.deleted_at = Set(Some(now.into()));

    save_active_venue(&db, venue).await?;

    Ok(())
}
