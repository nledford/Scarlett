use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use async_trait::async_trait;

use crate::schemas::DbTable;

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub struct Collection {
    pub id: i32,
    pub name: String,
    pub query: String,
}

#[async_trait]
impl DbTable for Collection {
    fn from_row(row: Row) -> Self {
        Collection {
            id: row.get(0),
            name: row.get(1),
            query: row.get(2),
        }
    }

    async fn get_all(pool: &Pool) -> Result<Vec<Self>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from collections order by name").await?;
        let results = client.query(&stmt, &[]).await?;

        let collections: Vec<Collection> = results.into_iter().map(Collection::from_row).collect();

        Ok(collections)
    }

    async fn get_by_id(id: i32, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from collections where id = $1").await?;
        let result = client.query_one(&stmt, &[&id]).await?;

        let collection = Collection::from_row(result);

        Ok(collection)
    }
}

impl Collection {
    pub async fn create(name: &str, query: &str, pool: &Pool) {
        unimplemented!()
    }

    pub async fn check_if_exists() {
        unimplemented!()
    }
}
