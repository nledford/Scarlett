use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug)]
pub struct PhotosAll {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub file_hash: String,
    pub rating: i32,
    pub date_created: NaiveDateTime,
    pub date_updated: NaiveDateTime,
    pub original_width: i32,
    pub original_height: i32,
    pub rotation: i32,
    pub ineligible_for_wallpaper: bool,
    pub anonymous_entities: bool,
    pub entities: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub wallpapers: Option<Vec<String>>,
}

impl PhotosAll {
    pub fn from_row(row: Row) -> Self {
        PhotosAll {
            id: row.get(0),
            file_path: row.get(1),
            file_name: row.get(2),
            file_hash: row.get(3),
            rating: row.get(4),
            date_created: row.get(5),
            date_updated: row.get(6),
            original_width: row.get(7),
            original_height: row.get(8),
            rotation: row.get(9),
            ineligible_for_wallpaper: row.get(10),
            anonymous_entities: row.get(11),
            entities: row.get(12),
            tags: row.get(13),
            wallpapers: row.get(14),
        }
    }
}
