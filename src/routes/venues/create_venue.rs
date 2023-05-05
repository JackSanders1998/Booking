use axum::{extract::State, http::StatusCode, Extension, Json};
use sea_orm::DatabaseConnection;

use crate::{
    queries::venue_queries, utilities::app_error::AppError,
};

use super::{create_venue_extractor::ValidateCreateVenue, ResponseDataVenue, ResponseVenue};

pub async fn create_venue(
    Extension(user): Extension<UserModel>,
    State(db): State<DatabaseConnection>,
    venue: ValidateCreateVenue,
) -> Result<(StatusCode, Json<ResponseDataVenue>), AppError> {
    let venue = venue_queries::create_venue(venue, &user, &db).await?;
    let response = ResponseVenue {
        id: venue.id,
        title: venue.title,
        description: venue.description,
        priority: venue.priority,
        completed_at: venue.completed_at.map(|time| time.to_string()),
    };

    Ok((
        StatusCode::CREATED,
        Json(ResponseDataVenue { data: response }),
    ))
}
