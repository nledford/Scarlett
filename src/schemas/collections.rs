use deadpool_postgres::{Pool, PoolError};

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
        let stmt = client
            .prepare("select * from collections order by name")
            .await?;
        let results = client.query(&stmt, &[]).await?;

        let collections: Vec<Collection> = results.into_iter().map(Collection::from_row).collect();

        Ok(collections)
    }

    async fn get_by_id(id: i32, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from collections where id = $1")
            .await?;
        let result = client.query_one(&stmt, &[&id]).await?;

        let collection = Collection::from_row(result);

        Ok(collection)
    }
}

impl Collection {
    pub async fn create(name: &str, query: &str, pool: &Pool) -> Result<Self, PoolError> {
        // first check if a collection already exists with the provided name
        let exists = Collection::check_if_exists(name, pool).await?;

        if exists {
            let collection = Collection::get_by_name(name, pool).await?;
            return Ok(collection);
        }

        // Assume collection does not exist

        let client = pool.get().await?;
        let stmt = client
            .prepare("insert into collections (name, query) values ($1, $2)")
            .await?;
        let _ = client.execute(&stmt, &[&name, &query]).await?;

        let collection = Collection::get_by_name(name, pool).await?;

        Ok(collection)
    }

    pub async fn update(collection: Collection, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;

        let stmt = client
            .prepare(
                "update collections \
                 set name = $1, query = $2 \
                 where id = $3",
            )
            .await?;

        let _ = client
            .execute(
                &stmt,
                &[&collection.name, &collection.query, &collection.id],
            )
            .await?;

        let result = Collection::get_by_id(collection.id, pool).await?;

        Ok(result)
    }

    pub async fn delete(id: i32, pool: &Pool) -> Result<String, PoolError> {
        let collection = Collection::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client
            .prepare("delete from collections where id = $1")
            .await?;
        let _ = client.execute(&stmt, &[&collection.id]).await?;

        Ok("Collection deleted successfully".to_string())
    }

    pub async fn get_by_name(name: &str, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from collections where name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&name]).await?;
        let collection = Collection::from_row(result);

        Ok(collection)
    }

    pub async fn check_if_exists(name: &str, pool: &Pool) -> Result<bool, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select count(*) from collections where name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&name]).await?;

        let count: i64 = result.get(0);

        Ok(count > 0)
    }
}
