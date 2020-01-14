use std::{fs, io};
use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use deadpool_postgres::{Client, Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotosAll {
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

impl PhotosAll {
    pub fn from_row(row: Row) -> Self {
        PhotosAll {
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

    pub async fn all_photos(pool: &Pool) -> Result<Vec<PhotosAll>, PoolError> {
        let client: Client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos_all").await?;
        let rows = client.query(&stmt, &[]).await?;

        let photos = rows
            .into_iter()
            .map(PhotosAll::from_row)
            .collect::<Vec<PhotosAll>>();

        Ok(photos)
    }
}

// PHOTO STATS *************************************************************************************

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: i64,
    pub pending_delete: i64,
    pub hidden: i64,
    pub neutral: i64,
    pub wallpaper_candidates: i64,
    pub favorites: i64,
    pub total_kept: i64,
    pub total: i64,
}

impl PhotosStats {
    pub fn from_row(row: Row) -> Self {
        PhotosStats {
            unrated: row.get(0),
            pending_delete: row.get(1),
            hidden: row.get(2),
            neutral: row.get(3),
            wallpaper_candidates: row.get(4),
            favorites: row.get(5),
            total_kept: row.get(6),
            total: row.get(7),
        }
    }

    pub async fn get_photos_stats(pool: &Pool) -> Result<PhotosStats, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT unrated, pending_delete, hidden, neutral, wallpaper_candidates, favorites, total_kept, total FROM photos_stats").await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = PhotosStats::from_row(result);

        Ok(stats)
    }
}
