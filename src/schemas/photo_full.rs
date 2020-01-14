use chrono::NaiveDateTime;
use deadpool_postgres::{Client, Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoFull {
    pub id: i32,
    pub file_path: String,
    pub folder: String,
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

impl PhotoFull {
    pub fn from_row(row: Row) -> Self {
        PhotoFull {
            id: row.get(0),
            file_path: row.get(1),
            folder: row.get(2),
            file_name: row.get(3),
            file_hash: row.get(4),
            rating: row.get(5),
            date_created: row.get(6),
            date_updated: row.get(7),
            original_width: row.get(8),
            original_height: row.get(9),
            rotation: row.get(10),
            ineligible_for_wallpaper: row.get(11),
            anonymous_entities: row.get(12),
            entities: row.get(13),
            tags: row.get(14),
            wallpapers: row.get(15),
        }
    }

    pub async fn all_photos(pool: &Pool) -> Result<Vec<PhotoFull>, PoolError> {
        let client: Client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos_all").await?;
        let rows = client.query(&stmt, &[]).await?;

        let photos = rows
            .into_iter()
            .map(PhotoFull::from_row)
            .collect::<Vec<PhotoFull>>();

        Ok(photos)
    }
}
