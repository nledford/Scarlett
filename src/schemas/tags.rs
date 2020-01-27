use crate::schemas::DbTable;
use async_trait::async_trait;
use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tag {
    pub id: i32,
    pub tag_name: String,
}

#[async_trait]
impl DbTable for Tag {
    fn from_row(row: Row) -> Self {
        Tag {
            id: row.get(0),
            tag_name: row.get(1),
        }
    }

    async fn get_all(pool: &Pool) -> Result<Vec<Tag>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM tags").await?;
        let results = client.query(&stmt, &[]).await?;
        let tags: Vec<Tag> = results.into_iter().map(Tag::from_row).collect();

        Ok(tags)
    }

    async fn get_by_id(id: i32, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM tags WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&id]).await?;
        let tag = Tag::from_row(result);

        Ok(tag)
    }
}

impl Tag {
    pub async fn create(tag_name: &str, pool: &Pool) -> Result<Tag, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("INSERT INTO tags (tag_name) VALUES($1)")
            .await?;
        let _ = client.execute(&stmt, &[&tag_name]).await?;

        let tag = Tag::get_by_name(tag_name, pool).await?;

        Ok(tag)
    }

    pub async fn get_by_name(tag_name: &str, pool: &Pool) -> Result<Tag, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM tags WHERE tag_name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&tag_name]).await?;
        let tag = Tag::from_row(result);

        Ok(tag)
    }

    pub async fn update(tag: Tag, pool: &Pool) -> Result<Tag, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("UPDATE tags SET tag_name = $1 WHERE id = $2")
            .await?;
        let _ = client.execute(&stmt, &[&tag.tag_name, &tag.id]).await?;

        let updated_tag = Tag::get_by_id(tag.id, pool).await?;

        Ok(updated_tag)
    }

    pub async fn delete(id: i32, pool: &Pool) -> Result<String, PoolError> {
        let tag_to_delete = Tag::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM tags WHERE id = $1").await?;
        let _ = client.execute(&stmt, &[&tag_to_delete.id]);

        Ok("Tag deleted successfully".to_string())
    }
}
