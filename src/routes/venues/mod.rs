pub mod create_venue;
pub mod create_venue_extractor;
pub mod delete_venue;
pub mod get_all_venues;
pub mod get_one_venue;
pub mod update_venues;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RequestVenue {
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub priority: Option<Option<String>>,
    pub title: Option<String>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub description: Option<Option<String>>,
    #[serde(
        default,                                    // <- important for deserialization
        skip_serializing_if = "Option::is_none",    // <- important for serialization
        with = "::serde_with::rust::double_option",
    )]
    pub completed_at: Option<Option<DateTime<FixedOffset>>>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseVenue {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub completed_at: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataVenue {
    pub data: ResponseVenue,
}

#[derive(Serialize, Deserialize)]
pub struct ResponseDataVenues {
    pub data: Vec<ResponseVenue>,
}
