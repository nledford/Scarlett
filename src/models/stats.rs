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
