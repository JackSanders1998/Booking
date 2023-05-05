use crate::{
    app_state::AppState,
    routes::{
        hello_world::hello_world,
        venues::{
            create_venue::create_venue,
            delete_venue::soft_delete_venue,
            get_all_venues::get_all_venues,
            get_one_venue::get_one_venue,
            update_venues::{mark_completed, mark_uncompleted, update_venue},
        },
        users::{create_user::create_user, login::login, logout::logout},
    },
};
use axum::{
    middleware,
    routing::{delete, get, patch, post, put},
    Router,
};

pub fn create_router(app_state: AppState) -> Router {
    Router::new()
        .route("/api/v1/venues", post(create_venue))
        .route("/api/v1/venues", get(get_all_venues))
        .route("/api/v1/venues/:venue_id", get(get_one_venue))
        .route("/api/v1/venues/:venue_id/completed", put(mark_completed))
        .route("/api/v1/venues/:venue_id/uncompleted", put(mark_uncompleted))
        .route("/api/v1/venues/:venue_id", patch(update_venue))
        .route("/api/v1/venues/:venue_id", delete(soft_delete_venue))
        .route("/", get(hello_world))
        .with_state(app_state)
}