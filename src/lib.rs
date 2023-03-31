use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::atomic::{AtomicUsize, Ordering},
};

#[cfg(feature = "persist")]
use tokio::fs;

/// Represents a single venue item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct VenueItem {
    pub title: String,
    pub description: String,
    pub address: String,
    pub published: bool,
}

/// DTO for patching a venue item
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct UpdateVenueItem {
    pub title: Option<String>,
    pub description: Option<String>,
    pub address: Option<String>,
    pub published: Option<bool>,
}

/// Represents a venue item with an id
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IdentifiableVenueItem {
    pub id: usize,

    #[serde(flatten)]
    pub item: VenueItem,
}

impl IdentifiableVenueItem {
    pub fn new(id: usize, item: VenueItem) -> IdentifiableVenueItem {
        IdentifiableVenueItem { id, item }
    }
}

/// Parameters for pagination
///
/// Used to demonstrate handling of query parameters.
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

/// Error type for the venue items store
#[derive(thiserror::Error, Debug)]
pub enum VenueStoreError {
    #[error("persistent data store error")]
    FileAccessError(#[from] std::io::Error),
    #[error("serialization error")]
    SerializationError(#[from] serde_json::error::Error),
}

/// Venue items store
#[derive(Default)]
pub struct VenueStore {
    store: HashMap<usize, IdentifiableVenueItem>,
    id_generator: AtomicUsize,
}
impl VenueStore {
    pub fn from_hashmap(store: HashMap<usize, IdentifiableVenueItem>) -> Self {
        let id_generator = AtomicUsize::new(store.keys().max().map(|v| v + 1).unwrap_or(0));
        VenueStore { store, id_generator }
    }

    /// Get list of venue items
    ///
    /// Supports pagination.
    pub fn get_venues(&self, pagination: Pagination) -> Vec<IdentifiableVenueItem> {
        self.store
            .values()
            .skip(pagination.offset.unwrap_or(0))
            .take(pagination.limit.unwrap_or(usize::MAX))
            .cloned()
            .collect::<Vec<_>>()
    }

    /// Get a single venue item by id
    pub fn get_venue(&self, id: usize) -> Option<&IdentifiableVenueItem> {
        self.store.get(&id)
    }

    /// Create a new venue item
    pub fn add_venue(&mut self, venue: VenueItem) -> IdentifiableVenueItem {
        let id = self.id_generator.fetch_add(1, Ordering::Relaxed);
        let new_item = IdentifiableVenueItem::new(id, venue);
        self.store.insert(id, new_item.clone());
        new_item
    }

    /// Remove a venue item by id
    pub fn remove_venue(&mut self, id: usize) -> Option<IdentifiableVenueItem> {
        self.store.remove(&id)
    }

    /// Patch a venue item by id
    pub fn update_venue(&mut self, id: &usize, venue: UpdateVenueItem) -> Option<&IdentifiableVenueItem> {
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

impl From<VenueStore> for HashMap<usize, IdentifiableVenueItem> {
    fn from(value: VenueStore) -> Self {
        value.store
    }
}
