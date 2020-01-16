use deadpool_postgres::{Pool, PoolError};
use tokio_postgres::Row;

use async_trait::async_trait;

pub mod entity;
pub mod new_photo;
pub mod photo;
pub mod photo_full;
pub mod tags;

// TRAITS ******************************************************************************************

#[async_trait]
pub trait DbTable {
    fn from_row(row: Row) -> Self;

    async fn get_all(pool: &Pool) -> Result<Vec<Self>, PoolError>
    where
        Self: std::marker::Sized;

    async fn get_by_id(id: i64, pool: &Pool) -> Result<Self, PoolError>
    where
        Self: std::marker::Sized;
}

#[async_trait]
pub trait DbView {
    fn from_row(row: Row) -> Self;

    async fn get_all(pool: &Pool) -> Result<Vec<Self>, PoolError>
    where
        Self: std::marker::Sized;

    async fn get_by_id(id: i64, pool: &Pool) -> Result<Self, PoolError>
    where
        Self: std::marker::Sized;
}

pub trait Paginated {
    fn from_paginated_row(row: Row) -> (Self, i64)
    where
        Self: std::marker::Sized;
}

// REFRESH `photo_order` MATERIALIZED VIEW *********************************************************

pub async fn reset_seed(pool: &Pool) -> Result<(), PoolError> {
    let client = pool.get().await?;

    let stmt = client
        .prepare("REFRESH MATERIALIZED VIEW photo_ordering")
        .await?;
    let _ = client.execute(&stmt, &[]).await?;

    Ok(())
}
