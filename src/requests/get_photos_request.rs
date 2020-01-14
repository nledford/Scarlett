use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct GetPhotosRequest {
    page: Option<i32>,
    page_size: Option<i32>,
}

impl GetPhotosRequest {
    pub fn get_page(&self) -> i32 {
        if self.page.is_some() {
            let page = self.page.unwrap();
            if page <= 0 {
                1
            } else {
                page
            }
        } else {
            1
        }
    }

    pub fn get_page_size(&self) -> i32 {
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
