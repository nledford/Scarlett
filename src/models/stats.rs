use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: Option<i64>,
    pub pending_delete: Option<i64>,
    pub hidden: Option<i64>,
    pub neutral: Option<i64>,
    pub wallpaper_candidates: Option<i64>,
    pub favorites: Option<i64>,
    pub with_entities: Option<i64>,
    pub with_tags: Option<i64>,
    pub with_wallpaper: Option<i64>,
    pub total_kept: Option<i64>,
    pub total: Option<i64>,
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
            with_entities: row.get(6),
            with_tags: row.get(7),
            with_wallpaper: row.get(8),
            total_kept: row.get(9),
            total: row.get(10),
        }
    }

    pub async fn get_photos_stats(pool: &Pool) -> Result<PhotosStats, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT unrated, \
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
                                                    FROM photos_stats").await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = PhotosStats::from_row(result);

        Ok(stats)
    }
}
