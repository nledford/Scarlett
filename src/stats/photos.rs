use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: i64,
    pub pending_delete: i64,
    pub hidden: i64,
    pub neutral: i64,
    pub wallpaper_candidates: i64,
    pub favorites: i64,
    pub with_entities: i64,
    pub with_tags: i64,
    pub with_wallpaper: i64,
    pub total_kept: i64,
    pub total: i64,
}

impl PhotosStats {
    pub fn from_row(row: Row) -> Self {
        PhotosStats {
            unrated: row.try_get(0).unwrap_or(0),
            pending_delete: row.try_get(1).unwrap_or(0),
            hidden: row.try_get(2).unwrap_or(0),
            neutral: row.try_get(3).unwrap_or(0),
            wallpaper_candidates: row.try_get(4).unwrap_or(0),
            favorites: row.try_get(5).unwrap_or(0),
            with_entities: row.try_get(6).unwrap_or(0),
            with_tags: row.try_get(7).unwrap_or(0),
            with_wallpaper: row.try_get(8).unwrap_or(0),
            total_kept: row.try_get(9).unwrap_or(0),
            total: row.try_get(10).unwrap_or(0),
        }
    }

    pub async fn get_photos_stats(pool: &Pool) -> Result<PhotosStats, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT unrated, \
                 pending_delete, \
                 hidden, \
                 neutral, \
                 wallpaper_candidates, \
                 favorites, \
                 with_entities, \
                 with_tags, \
                 with_wallpaper, \
                 total_kept, \
                 total \
                 FROM photos_stats",
            )
            .await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = PhotosStats::from_row(result);

        Ok(stats)
    }
}
