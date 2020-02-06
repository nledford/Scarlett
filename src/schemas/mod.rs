use deadpool_postgres::Pool;

use crate::types::DbSingleResult;

pub mod collections;
pub mod directory_tree;
pub mod entity;
pub mod new_photo;
pub mod photo;
pub mod photo_full;
pub mod tags;
pub mod wallpaper_sizes;

// REFRESH `photo_order` MATERIALIZED VIEW *********************************************************

pub async fn reset_seed(pool: &Pool) -> DbSingleResult<()> {
    let client = pool.get().await?;

    let stmt = client
        .prepare("REFRESH MATERIALIZED VIEW photo_ordering")
        .await?;
    let _ = client.execute(&stmt, &[]).await?;

    Ok(())
}
