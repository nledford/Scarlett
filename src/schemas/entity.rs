use async_trait::async_trait;
use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::schemas::DbTable;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Entity {
    pub id: i32,
    pub entity_name: String,
    pub alternate_names: Option<Vec<String>>,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub favorite: bool,
    pub profile_photo_id: Option<i32>,
}

#[async_trait]
impl DbTable for Entity {
    fn from_row(row: Row) -> Self {
        Entity {
            id: row.get(0),
            entity_name: row.get(1),
            alternate_names: row.get(2),
            instagram_username: row.get(3),
            twitter_username: row.get(4),
            favorite: row.get(5),
            profile_photo_id: row.get(6),
        }
    }

    async fn get_all(pool: &Pool) -> Result<Vec<Self>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM entity").await?;
        let results = client.query(&stmt, &[]).await?;

        let entities: Vec<Entity> = results.into_iter().map(Entity::from_row).collect();

        Ok(entities)
    }

    async fn get_by_id(id: i64, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT * \
                 FROM entity \
                 WHERE id = $1",
            )
            .await?;
        let result = client.query_one(&stmt, &[&id]).await?;

        let entity = Entity::from_row(result);

        Ok(entity)
    }
}

impl Entity {
    pub async fn update(entity: Entity, pool: &Pool) -> Result<Entity, PoolError> {
        let client = pool.get().await?;

        let stmt = client
            .prepare(
                "UPDATE entity\
                 SET entity_name = $1,\
                 alternate_names = $2,\
                 instagram_username = $3,\
                 twitter_username = $4,\
                 favorite = $5,\
                 profile_photo_id = $6\
                 WHERE id = $7",
            )
            .await?;

        let _ = client
            .execute(
                &stmt,
                &[
                    &entity.entity_name,
                    &entity.alternate_names,
                    &entity.instagram_username,
                    &entity.twitter_username,
                    &entity.favorite,
                    &entity.profile_photo_id,
                    &entity.id,
                ],
            )
            .await?;

        let result = Entity::get_by_id(entity.id as i64, pool).await?;

        Ok(result)
    }

    pub async fn delete(id: i64, pool: &Pool) -> Result<String, PoolError> {
        let entity = Entity::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM entity WHERE id = $1").await?;
        let _ = client.execute(&stmt, &[&id]).await?;

        Ok("Entity deleted successfully".to_string())
    }
}
