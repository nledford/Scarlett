use serde::Deserialize;

use crate::utils::strings;

#[derive(Debug, Clone, Deserialize)]
pub struct GetPhotosRequest {
    // pagination
    page: Option<i64>,
    page_size: Option<i64>,

    // sorting
    sort_by: Option<String>,

    // collections
    pub collection_id: Option<i32>,
    // filters
}

impl GetPhotosRequest {
    pub fn get_page(&self) -> i64 {
        if self.page.is_none() {
            return 1
        }

        let page = self.page.unwrap();
        if page <= 0 {
            1
        } else {
            page
        }
    }

    pub fn get_page_size(&self) -> i64 {
        if self.page_size.is_none() {
            return 100
        }

        let size = self.page_size.unwrap();

        if size <= 0 {
            100
        } else {
            size
        }
    }

    pub fn get_sort_by(&self) -> Option<Vec<String>> {
        let valid_sort_options = vec!["id", "date_created", "date_updated", "file_name", "folder"];

        if self.sort_by.is_none() {
            return None
        }

        let temp = self
            .sort_by
            .clone()
            .unwrap()
            .split(',')
            .into_iter()
            .filter(|item| {
                valid_sort_options
                    .contains(&strings::get_category_from_sort(&item.to_string()).as_str())
            })
            .map(String::from)
            .collect::<Vec<String>>();

        if temp.len() > 0 {
            return Some(temp);
        } else {
            None
        }
    }

    pub fn has_collection_or_filters(&self) -> bool {
        if self.collection_id.is_some() {
            return true
        }

        false
    }
}
