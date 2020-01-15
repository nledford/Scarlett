use deadpool_postgres::{Pool, PoolError};

pub mod new_photo;
pub mod photo;
pub mod photo_full;

pub async fn reset_seed(pool: &Pool) -> Result<(), PoolError> {
    let client = pool.get().await?;

    let stmt = client
        .prepare("REFRESH MATERIALIZED VIEW photo_ordering")
        .await?;
    let _ = client.execute(&stmt, &[]).await?;

    Ok(())
}
