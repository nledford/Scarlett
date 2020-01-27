use crate::pagination::links::Links;
use crate::pagination::page_metadata::PageMetadata;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page<T> {
    pub metadata: PageMetadata,
    pub links: Links,
    pub data: T,
}

impl<T> Page<T> {
    pub fn new(metadata: PageMetadata, links: Links, data: T) -> Page<T> {
        Page {
            metadata,
            links,
            data,
        }
    }
}
