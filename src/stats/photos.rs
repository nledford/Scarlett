use deadpool_postgres::Pool;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::errors::ServiceError;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PhotosStats {
    pub unrated: i64,
    pub unrated_percent: Decimal,

    pub hidden: i64,
    pub hidden_percent: Decimal,

    pub neutral: i64,
    pub neutral_percent: Decimal,

    pub wallpaper_candidates: i64,
    pub wc_percent: Decimal,

    pub favorites: i64,
    pub favorites_percent: Decimal,

    pub pending_delete: i64,
    pub pending_delete_percent: Decimal,

    pub total_kept: i64,
    pub kept_percent: Decimal,

    pub total: i64,

    pub with_entities: i64,
    pub with_entities_percent: Decimal,

    pub with_tags: i64,
    pub with_tags_percent: Decimal,

    pub with_wallpaper: i64,
    pub with_wallpaper_percent: Decimal,
}

impl PhotosStats {
    pub fn from_row(row: Row) -> Self {
        let total: i64 = row.try_get("total").unwrap_or(0);

        let unrated: i64 = row.try_get(0).unwrap_or(0);
        let unrated_percent = divide(unrated, total);

        let hidden = row.try_get(2).unwrap_or(0);
        let hidden_percent = divide(hidden, total);

        let neutral = row.try_get(4).unwrap_or(0);
        let neutral_percent = divide(neutral, total);

        let wallpaper_candidates = row.try_get(6).unwrap_or(0);
        let wc_percent = divide(wallpaper_candidates, total);

        let favorites = row.try_get(8).unwrap_or(0);
        let favorites_percent = divide(favorites, total);

        let with_entities = row.try_get(10).unwrap_or(0);
        let with_entities_percent = divide(with_entities, total);

        let with_tags = row.try_get(12).unwrap_or(0);
        let with_tags_percent = divide(with_tags, total);

        let with_wallpaper = row.try_get(14).unwrap_or(0);
        let with_wallpaper_percent = divide(with_wallpaper, total);

        let pending_delete = row.try_get(18).unwrap_or(0);
        let pending_delete_percent = divide(pending_delete, total);

        let total_kept = row.try_get("total_kept").unwrap_or(0);
        let kept_percent = divide(total_kept, total);

        PhotosStats {
            unrated,
            unrated_percent,

            hidden,
            hidden_percent,

            neutral,
            neutral_percent,

            wallpaper_candidates,
            wc_percent,

            favorites,
            favorites_percent,

            with_entities,
            with_entities_percent,

            with_tags,
            with_tags_percent,

            with_wallpaper,
            with_wallpaper_percent,

            total_kept,
            kept_percent,

            pending_delete,
            pending_delete_percent,

            total,
        }
    }

    pub async fn get_photos_stats(pool: &Pool) -> Result<PhotosStats, ServiceError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos_stats").await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = PhotosStats::from_row(result);

        Ok(stats)
    }
}

fn divide(a: i64, b: i64) -> Decimal {
    let scale = 0;
    (Decimal::new(a, scale) / Decimal::new(b, scale)) * Decimal::new(100, scale)
}
