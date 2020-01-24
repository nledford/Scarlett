use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetPhotosRequest {
    // pagination
    page: Option<i64>,
    page_size: Option<i64>,

    // sorting

    // collections
    pub collection_id: Option<i32>,

    // filters
}

impl GetPhotosRequest {
    pub fn get_page(&self) -> i64 {
        if self.page.is_some() {
            let page = self.page.unwrap();
            if page < 0 {
                0
            } else {
                page
            }
        } else {
            0
        }
    }

    pub fn get_page_size(&self) -> i64 {
        if self.page_size.is_some() {
            let size = self.page_size.unwrap();

            if size <= 0 {
                100
            } else {
                size
            }
        } else {
            100
        }
    }
}
