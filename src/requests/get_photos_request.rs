use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct GetPhotosRequest {
    position: Option<i64>,
    page_size: Option<i64>,
}

impl GetPhotosRequest {
    pub fn get_position(&self) -> i64 {
        if self.position.is_some() {
            let position = self.position.unwrap();
            if position < 0 {
                0
            } else {
                position
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
