use deadpool_postgres::{Pool, PoolError};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: i64,
    pub unrated_percent: Decimal,

    pub pending_delete: i64,
    pub pending_delete_percent: Decimal,

    pub hidden: i64,
    pub hidden_percent: Decimal,

    pub neutral: i64,
    pub neutral_percent: Decimal,

    pub wallpaper_candidates: i64,
    pub wc_percent: Decimal,

    pub favorites: i64,
    pub favorites_percent: Decimal,

    pub with_entities: i64,
    pub with_entities_percent: Decimal,

    pub with_tags: i64,
    pub with_tags_percent: Decimal,

    pub with_wallpaper: i64,
    pub with_wallpaper_percent: Decimal,

    pub total_kept: i64,
    pub kept_percent: Decimal,

    pub total: i64,
}

impl PhotosStats {
    pub fn from_row(row: Row) -> Self {
        PhotosStats {
            unrated: row.try_get(0).unwrap_or(0),
            unrated_percent: row.try_get(1).unwrap_or(Decimal::default()),

            hidden: row.try_get(2).unwrap_or(0),
            hidden_percent: row.try_get(3).unwrap_or(Decimal::default()),

            neutral: row.try_get(4).unwrap_or(0),
            neutral_percent: row.try_get(5).unwrap_or(Decimal::default()),

            wallpaper_candidates: row.try_get(6).unwrap_or(0),
            wc_percent: row.try_get(7).unwrap_or(Decimal::default()),

            favorites: row.try_get(8).unwrap_or(0),
            favorites_percent: row.try_get(9).unwrap_or(Decimal::default()),

            with_entities: row.try_get(10).unwrap_or(0),
            with_entities_percent: row.try_get(11).unwrap_or(Decimal::default()),

            with_tags: row.try_get(12).unwrap_or(0),
            with_tags_percent: row.try_get(13).unwrap_or(Decimal::default()),

            with_wallpaper: row.try_get(14).unwrap_or(0),
            with_wallpaper_percent: row.try_get(15).unwrap_or(Decimal::default()),

            total_kept: row.try_get(16).unwrap_or(0),
            kept_percent: row.try_get(17).unwrap_or(Decimal::default()),

            pending_delete: row.try_get(18).unwrap_or(0),
            pending_delete_percent: row.try_get(19).unwrap_or(Decimal::default()),

            total: row.try_get(20).unwrap_or(0),
        }
    }

    pub async fn get_photos_stats(pool: &Pool) -> Result<PhotosStats, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT * FROM photos_stats",
            )
            .await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = PhotosStats::from_row(result);

        Ok(stats)
    }
}
