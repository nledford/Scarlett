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
    folder: Option<String>,
    exclude_ratings: Option<String>,
}

impl GetPhotosRequest {
    pub fn get_page(&self) -> i64 {
        let page = self.page.unwrap_or(1);
        if page <= 0 {
            1
        } else {
            page
        }
    }

    pub fn get_page_size(&self) -> i64 {
        let size = self.page_size.unwrap_or(100);
        if size <= 0 {
            100
        } else {
            size
        }
    }

    pub fn get_sort_by(&self) -> Option<Vec<String>> {
        let valid_sort_options = vec!["id", "date_created", "date_updated", "file_name", "folder"];

        let temp = match &self.sort_by {
            Some(val) => val,
            None => return None,
        };

        let temp: Vec<String> = temp
            .split(',')
            .filter(|item| valid_sort_options.contains(&strings::get_category_from_sort(*item)))
            .map(String::from)
            .collect();

        if temp.is_empty() {
            None
        } else {
            Some(temp)
        }
    }

    // filters

    pub fn get_folder(&self) -> String {
        self.folder
            .to_owned()
            .unwrap_or("/".to_string())
    }

    pub fn get_exclude_ratings(&self) -> Option<Vec<String>> {
        let valid_ratings = vec!["0", "1", "2", "3", "4", "5"];

        let temp = match &self.exclude_ratings {
            Some(val) => val,
            None => return None,
        };

        let temp: Vec<String> = temp
            .split(',')
            .filter(|item| valid_ratings.contains(item))
            .map(String::from)
            .collect();

        if temp.is_empty() {
            None
        } else {
            Some(temp)
        }
    }

    // misc

    pub fn has_collection_or_filters(&self) -> bool {
        // collections first
        if self.collection_id.is_some() {
            return true
        }

        // filters second

        if self.folder.is_some() {
            return true
        }

        // default value last
        false
    }
}
