use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PageMetadata {
    pub page: i64,
    pub page_size: i64,
    pub page_count: i64,
    pub total_items: i64,
}

impl PageMetadata {
    pub fn new(page: i64, page_size: i64, total_items: i64) -> Self {
        let page_count = (total_items as f64 / page_size as f64).ceil() as i64;

        PageMetadata {
            page,
            page_size,
            page_count,
            total_items,
        }
    }
}
