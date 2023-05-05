pub mod app_state;
mod queries;
mod router;
mod routes;
pub mod utilities;

use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(feature = "persist")]
use tokio::fs;
use tracing::info;

/// Represents a single venue
#[derive(Serialize, Deserialize, Debug, Clone, IntoParams, ToSchema)]
pub struct Venue {
    pub title: String,
    pub description: String,
    pub address: String,
    pub published: bool,
}

/// DTO for patching a venue
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateVenue {
    pub title: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub published: Option<bool>,
}

/// Represents a venue with an id
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifiableVenue {
    pub id: usize,

    #[serde(flatten)]
    pub item: Venue,
}

impl IdentifiableVenue {
    pub fn new(id: usize, item: Venue) -> IdentifiableVenue {
        IdentifiableVenue { id, item }
    }
}

/// Parameters for pagination
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}
impl Pagination {
    pub fn new(offset: Option<usize>, limit: Option<usize>) -> Pagination {
        Pagination { offset, limit }
    }
}

/// Error type for the venue store
#[derive(thiserror::Error, Debug)]
pub enum VenueStoreError {
    #[error("persistent data store error")]
    FileAccessError(#[from] std::io::Error),
    #[error("serialization error")]
    SerializationError(#[from] serde_json::error::Error),
}

/// Venue store
#[derive(Default)]
pub struct VenueStore {
    store: HashMap<usize, IdentifiableVenue>,
    id_generator: AtomicUsize,
}
impl VenueStore {
    pub fn from_hashmap(store: HashMap<usize, IdentifiableVenue>) -> Self {
        let id_generator = AtomicUsize::new(store.keys().max().map(|v| v + 1).unwrap_or(0));
        VenueStore { store, id_generator }
    }

    /// Get list of venues with pagination support
    pub fn get_venues(&self, pagination: Pagination) -> Vec<IdentifiableVenue> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    /// Get a single venue by id
    pub fn get_venue(&self, id: usize) -> Option<&IdentifiableVenue> {
        self.store.get(&id)
    }

    /// Create a new venue
    pub fn add_venue(&mut self, venue: Venue) -> IdentifiableVenue {
        info!("Adding new venue: {:?}", venue);
        let id = self.id_generator.fetch_add(1, Ordering::Relaxed);
        let new_item = IdentifiableVenue::new(id, venue);
        self.store.insert(id, new_item.clone());
        new_item
    }

    /// Remove a venue by id
    pub fn remove_venue(&mut self, id: usize) -> Option<IdentifiableVenue> {
        self.store.remove(&id)
    }

    /// Patch a venue by id
    pub fn update_venue(&mut self, id: &usize, venue: UpdateVenue) -> Option<&IdentifiableVenue> {
        if let Some(item) = self.store.get_mut(id) {
            if let Some(title) = venue.title {
                item.item.title = title;
            }
            if let Some(description) = venue.description {
                item.item.description = description;
            }
            if let Some(address) = venue.address {
                item.item.address = address;
            }
            if let Some(published) = venue.published {
                item.item.published = published;
            }

            Some(item)
        } else {
            None
        }
    }
}

impl From<VenueStore> for HashMap<usize, IdentifiableVenue> {
    fn from(value: VenueStore) -> Self {
        value.store
    }
}
