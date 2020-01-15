use std::fs::File;
use std::io;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use deadpool_postgres::{Pool, PoolError};

use sha3::{Digest, Sha3_256};

use crate::schemas::photo::Photo;

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
        let dt_created = system_time_to_date_time(dt_created).naive_utc();

        let dim = image::image_dimensions(&path).unwrap();
        let width = dim.0 as i32;
        let height = dim.1 as i32;

        NewPhoto {
            file_name: get_file_name(&path),
            file_hash: calculate_sha3_hash(&path),
            file_path: path,
            date_created: dt_created,
            original_width: width,
            original_height: height,
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

        let stmt = "INSERT INTO photos (file_path, file_name, file_hash, rating, date_created, date_updated, original_width, original_height, rotation, ineligible_for_wallpaper, anonymous_entities) \
                          VALUES ($1, $2, $3, 0, $4, $4, $5, $6, 0, false, false)";
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

            count += result;
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
