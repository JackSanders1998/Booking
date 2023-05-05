use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        venues::{self, Entity as Venues, Model as VenueModel},
        users::Model as UserModel,
    },
    routes::venues::create_venue_extractor::ValidateCreateVenue,
    utilities::app_error::AppError,
};

pub async fn create_venue(
    venue: ValidateCreateVenue,
    user: &UserModel,
    db: &DatabaseConnection,
) -> Result<VenueModel, AppError> {
    let new_venue = venues::ActiveModel {
        priority: Set(venue.priority),
        title: Set(venue.title.unwrap()),
        description: Set(venue.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    save_active_venue(db, new_venue).await
}

pub async fn find_venue_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<VenueModel, AppError> {
    let venue = Venues::find_by_id(id)
        .filter(venues::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting venue by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your venue",
            )
        })?;

    venue.ok_or_else(|| {
        eprintln!("Could not find venue by id");
        AppError::new(StatusCode::NOT_FOUND, "not found")
    })
}

pub async fn save_active_venue(
    db: &DatabaseConnection,
    venue: venues::ActiveModel,
) -> Result<VenueModel, AppError> {
    let venue = venue.save(db).await.map_err(|error| {
        eprintln!("Error saving venue: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving venue")
    })?;

    convert_active_to_model(venue)
}

pub async fn get_all_venues(
    db: &DatabaseConnection,
    user_id: i32,
    get_deleted: bool,
) -> Result<Vec<VenueModel>, AppError> {
    let mut query = Venues::find().filter(venues::Column::UserId.eq(Some(user_id)));

    if !get_deleted {
        query = query.filter(venues::Column::DeletedAt.is_null());
    }

    query.all(db).await.map_err(|error| {
        eprintln!("Error getting all venues: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all venues")
    })
}

pub async fn get_default_venues(db: &DatabaseConnection) -> Result<Vec<VenueModel>, AppError> {
    Venues::find()
        .filter(venues::Column::IsDefault.eq(Some(true)))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default venues: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default venues",
            )
        })
}

fn convert_active_to_model(active_venue: venues::ActiveModel) -> Result<VenueModel, AppError> {
    active_venue.try_into_model().map_err(|error| {
        eprintln!("Error converting venue active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
