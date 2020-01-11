use std::fs::File;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use deadpool_postgres::{Client, Pool, PoolError};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};
use tokio_postgres::Row;

// `photos` table ************************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
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
}

impl Photo {
    pub fn from_row(row: Row) -> Self {
        Photo {
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
        }
    }

    pub async fn update_photo(updated_photo: Photo, pool: &Pool) -> Result<Self, PoolError> {
        let mut updated = updated_photo.clone();
        updated.date_updated = Utc::now().naive_utc();

        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE photos
                                    SET file_path = $1,
                                        file_name = $2,
                                        file_hash = $3,
                                        rating = $4,
                                        date_created = $5,
                                        date_updated = $6,
                                        original_width = $7,
                                        original_height = $8,
                                        rotation = $9,
                                        ineligible_for_wallpaper = $10,
                                        anonymous_entities = $11
                                    WHERE id = $12",
            )
            .await?;
        let _result = client
            .execute(
                &stmt,
                &[
                    &updated.file_path,
                    &updated.file_name,
                    &updated.file_hash,
                    &updated.rating,
                    &updated.date_created,
                    &updated.date_updated,
                    &updated.original_height,
                    &updated.original_width,
                    &updated.rotation,
                    &updated.ineligible_for_wallpaper,
                    &updated.anonymous_entities,
                    &updated.id,
                ],
            )
            .await?;

        let result = Photo::get_photo_by_id(updated.id as i64, pool).await?;

        Ok(result)
    }

    pub async fn get_photo_by_id(photo_id: i64, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&photo_id]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }

    pub async fn get_photo_by_name(name: &str, hash: &str, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM photos WHERE file_name = $1 AND file_path = $2")
            .await?;
        let result = client.query_one(&stmt, &[&name, &hash]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }

    pub async fn delete_photo(photo_id: i64, pool: &Pool) -> Result<String, PoolError> {
        let photo = Photo::get_photo_by_id(photo_id, &pool).await?;

        // attempt to delete photo
        fs::remove_file(&photo.file_path).expect("Could not delete file");

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM photos WHERE id = $1").await?;
        let _result = client.execute(&stmt, &[&photo_id]).await?;

        Ok("File deleted successfully!".to_string())
    }
}

// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
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

// NEW PHOTO ***************************************************************************************

#[derive(Clone)]
pub struct NewPhoto {
    pub file_path: String,
    pub file_name: String,
    pub file_hash: String,
    pub date_created: NaiveDateTime,
    pub original_height: i32,
    pub original_width: i32,
}

impl NewPhoto {
    pub fn new(path: String, dt_created: SystemTime) -> Self {
        let mut width = 0;
        let mut height = 0;

        let dt_created = system_time_to_date_time(dt_created).naive_utc();

        let dim = image::image_dimensions(&path).unwrap();
        width = dim.0;
        height = dim.1;

        NewPhoto {
            file_name: get_file_name(&path),
            file_hash: calculate_sha3_hash(&path),
            file_path: path,
            date_created: dt_created,
            original_width: width as i32,
            original_height: height as i32,
        }
    }

    pub async fn insert(&self, pool: &Pool) -> Result<Photo, PoolError> {
        let client = pool.get().await?;

        let stmt = client.prepare("INSERT INTO photos (file_path, file_name, file_hash, rating, date_created, date_updated, original_width, original_height, rotation, ineligible_for_wallpaper, anonymous_entities) \
        VALUES ($1, $2, $3, 0, $4, $4, $5, $6, 0, false, false) RETURNING id").await?;

        let result = client
            .query_one(
                &stmt,
                &[
                    &self.file_path,
                    &self.file_name,
                    &self.file_hash,
                    &self.date_created,
                    &self.original_width,
                    &self.original_height,
                ],
            )
            .await?;
        let result = Photo::get_photo_by_id(result.get(0), pool).await?;

        Ok(result)
    }

    pub async fn bulk_insert(new_photos: Vec<Self>, pool: &Pool) -> Result<u64, PoolError> {
        let client = pool.get().await?;

        let stmt = "INSERT INTO photos (file_path, file_name, file_hash, rating, date_created, date_updated, original_width, original_height, rotation, ineligible_for_wallpaper, anonymous_entities) VALUES ($1, $2, $3, 0, $4, $4, $5, $6, 0, false, false)";
        let mut count = 0;

        for photo in new_photos {
            let result = client
                .execute(
                    stmt,
                    &[
                        &photo.file_path,
                        &photo.file_name,
                        &photo.file_hash,
                        &photo.date_created,
                        &photo.original_width,
                        &photo.original_height,
                    ],
                )
                .await
                .unwrap();

            count = count + result;
        }

        Ok(count)
    }
}

fn system_time_to_date_time(t: SystemTime) -> DateTime<Utc> {
    let (sec, nsec) = match t.duration_since(UNIX_EPOCH) {
        Ok(dur) => (dur.as_secs() as i64, dur.subsec_nanos()),
        Err(e) => {
            let dur = e.duration();
            let (sec, nsec) = (dur.as_secs() as i64, dur.subsec_nanos());
            if nsec == 0 {
                (-sec, 0)
            } else {
                (-sec - 1, 1_000_000_000 - nsec)
            }
        }
    };
    Utc.timestamp(sec, nsec)
}

fn calculate_sha3_hash(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut hasher = Sha3_256::new();
    let _n = io::copy(&mut file, &mut hasher).unwrap();
    let hash = format!("{:x}", hasher.result());

    hash
}

fn get_hashed_path(path: &Path) -> String {
    let mut hasher = Sha3_256::new();
    hasher.input(path.to_str().unwrap().to_string().as_bytes());
    let hash = format!("{:x}", hasher.result());

    hash
}

fn get_file_name(path: &str) -> String {
    Path::new(path)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
}

// PHOTO STATS *************************************************************************************

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: i32,
    pub pending_delete: i32,
    pub hidden: i32,
    pub neutral: i32,
    pub wallpaper_candidates: i32,
    pub favorites: i32,
    pub total_kept: i32,
    pub total: i32,
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
