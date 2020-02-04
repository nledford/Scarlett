use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::errors::ServiceError;

#[derive(Serialize, Deserialize, Clone, Debug, PostgresMapper)]
#[pg_mapper(table = "entity")]
pub struct Entity {
    pub id: i32,
    pub entity_name: String,
    pub alternate_names: Option<Vec<String>>,
    pub instagram_username: Option<String>,
    pub twitter_username: Option<String>,
    pub favorite: bool,
    pub profile_photo_id: Option<i32>,
}

impl Entity {
    pub async fn get_all(pool: &Pool) -> Result<Vec<Self>, ServiceError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM entity").await?;
        let results = client.query(&stmt, &[]).await?;

        let entities: Vec<Entity> = results
            .into_iter()
            .map(|result| Entity::from_row(result).unwrap())
            .collect();

        Ok(entities)
    }

    pub async fn get_by_id(id: i32, pool: &Pool) -> Result<Self, ServiceError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "SELECT * \
                 FROM entity \
                 WHERE id = $1",
            )
            .await?;
        let result = client.query_one(&stmt, &[&id]).await?;

        let entity = Entity::from_row(result).unwrap();

        Ok(entity)
    }

    pub async fn create_simple(entity_name: &str, pool: &Pool) -> Result<Entity, ServiceError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("INSERT INTO entity (entity_name) VALUES ($1)")
            .await?;
        let _ = client.execute(&stmt, &[&entity_name]).await?;

        let entity = Entity::get_by_name(entity_name, pool).await?;

        Ok(entity)
    }

    pub async fn get_by_name(entity_name: &str, pool: &Pool) -> Result<Entity, ServiceError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM entity WHERE entity_name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&entity_name]).await?;
        let entity = Entity::from_row(result).unwrap();

        Ok(entity)
    }

    pub async fn update(entity: Entity, pool: &Pool) -> Result<Entity, ServiceError> {
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

        let result = Entity::get_by_id(entity.id, pool).await?;

        Ok(result)
    }

    pub async fn delete(id: i32, pool: &Pool) -> Result<String, ServiceError> {
        let entity = Entity::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM entity WHERE id = $1").await?;
        let _ = client.execute(&stmt, &[&entity.id]).await?;

        Ok("Entity deleted successfully".to_string())
    }

    pub async fn perform_search(q: String, pool: &Pool) -> Result<Vec<Self>, ServiceError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "select * \
            from sorted_entity \
            order by similarity(entity_name, $1) desc, sort_name \
            limit 5",
            )
            .await?;
        let results = client.query(&stmt, &[&q]).await?;

        let search_results: Vec<Entity> = results
            .into_iter()
            .map(|result| Entity::from_row(result).unwrap())
            .collect();

        Ok(search_results)
    }
}
