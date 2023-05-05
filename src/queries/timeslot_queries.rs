use axum::http::StatusCode;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set, TryIntoModel,
};

use crate::{
    database::{
        timeslots::{self, Entity as Timeslots, Model as TimeslotModel},
        users::Model as UserModel,
    },
    routes::timeslots::create_timeslot_extractor::ValidateCreateTimeslot,
    utilities::app_error::AppError,
};

pub async fn create_timeslot(
    timeslot: ValidateCreateTimeslot,
    user: &UserModel,
    db: &DatabaseConnection,
) -> Result<TimeslotModel, AppError> {
    let new_timeslot = timeslots::ActiveModel {
        priority: Set(timeslot.priority),
        title: Set(timeslot.title.unwrap()),
        description: Set(timeslot.description),
        user_id: Set(Some(user.id)),
        ..Default::default()
    };

    save_active_timeslot(db, new_timeslot).await
}

pub async fn find_timeslot_by_id(
    db: &DatabaseConnection,
    id: i32,
    user_id: i32,
) -> Result<TimeslotModel, AppError> {
    let timeslot = Timeslots::find_by_id(id)
        .filter(timeslots::Column::UserId.eq(Some(user_id)))
        .one(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting timeslot by id: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting your timeslot",
            )
        })?;

    timeslot.ok_or_else(|| {
        eprintln!("Could not find timeslot by id");
        AppError::new(StatusCode::NOT_FOUND, "not found")
    })
}

pub async fn save_active_timeslot(
    db: &DatabaseConnection,
    timeslot: timeslots::ActiveModel,
) -> Result<TimeslotModel, AppError> {
    let timeslot = timeslot.save(db).await.map_err(|error| {
        eprintln!("Error saving timeslot: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error saving timeslot")
    })?;

    convert_active_to_model(timeslot)
}

pub async fn get_all_timeslots(
    db: &DatabaseConnection,
    user_id: i32,
    get_deleted: bool,
) -> Result<Vec<TimeslotModel>, AppError> {
    let mut query = Timeslots::find().filter(timeslots::Column::UserId.eq(Some(user_id)));

    if !get_deleted {
        query = query.filter(timeslots::Column::DeletedAt.is_null());
    }

    query.all(db).await.map_err(|error| {
        eprintln!("Error getting all timeslots: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Error getting all timeslots")
    })
}

pub async fn get_default_timeslots(db: &DatabaseConnection) -> Result<Vec<TimeslotModel>, AppError> {
    Timeslots::find()
        .filter(timeslots::Column::IsDefault.eq(Some(true)))
        .all(db)
        .await
        .map_err(|error| {
            eprintln!("Error getting default timeslots: {:?}", error);
            AppError::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "Error getting default timeslots",
            )
        })
}

fn convert_active_to_model(active_timeslot: timeslots::ActiveModel) -> Result<TimeslotModel, AppError> {
    active_timeslot.try_into_model().map_err(|error| {
        eprintln!("Error converting timeslot active model to model: {:?}", error);
        AppError::new(StatusCode::INTERNAL_SERVER_ERROR, "Internal server error")
    })
}
