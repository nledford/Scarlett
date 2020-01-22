use std::fs;

use async_trait::async_trait;
use chrono::{NaiveDateTime, Utc};
use deadpool_postgres::{Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;

use crate::schemas::entity::Entity;
use crate::schemas::tags::Tag;
use crate::schemas::DbTable;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Photo {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub file_hash: String,
    pub rating: i32,
    pub date_created: NaiveDateTime,
    pub date_updated: NaiveDateTime,
    pub original_width: i32,
    pub original_height: i32,
    pub rotation: i32,
    pub ineligible_for_wallpaper: bool,
    pub anonymous_entities: bool,
}

#[async_trait]
impl DbTable for Photo {
    fn from_row(row: Row) -> Self {
        Photo {
            id: row.get(0),
            file_path: row.get(1),
            file_name: row.get(2),
            file_hash: row.get(3),
            rating: row.get(4),
            date_created: row.get(5),
            date_updated: row.get(6),
            original_width: row.get(7),
            original_height: row.get(8),
            rotation: row.get(9),
            ineligible_for_wallpaper: row.get(10),
            anonymous_entities: row.get(11),
        }
    }

    async fn get_all(pool: &Pool) -> Result<Vec<Self>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos").await?;
        let results = client.query(&stmt, &[]).await?;
        let photos: Vec<Photo> = results.into_iter().map(Photo::from_row).collect();

        Ok(photos)
    }

    async fn get_by_id(photo_id: i32, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&photo_id]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }
}

impl Photo {
    pub async fn update_photo(updated_photo: Photo, pool: &Pool) -> Result<Self, PoolError> {
        let mut updated = updated_photo.clone();
        updated.date_updated = Utc::now().naive_utc();

        let client = pool.get().await?;
        let stmt = client
            .prepare(
                "UPDATE photos
                                    SET file_path = $1,
                                        file_name = $2,
                                        file_hash = $3,
                                        rating = $4,
                                        date_created = $5,
                                        date_updated = $6,
                                        original_width = $7,
                                        original_height = $8,
                                        rotation = $9,
                                        ineligible_for_wallpaper = $10,
                                        anonymous_entities = $11
                                    WHERE id = $12",
            )
            .await?;
        let _result = client
            .execute(
                &stmt,
                &[
                    &updated.file_path,
                    &updated.file_name,
                    &updated.file_hash,
                    &updated.rating,
                    &updated.date_created,
                    &updated.date_updated,
                    &updated.original_height,
                    &updated.original_width,
                    &updated.rotation,
                    &updated.ineligible_for_wallpaper,
                    &updated.anonymous_entities,
                    &updated.id,
                ],
            )
            .await?;

        let result = Photo::get_by_id(updated.id, pool).await?;

        Ok(result)
    }

    pub async fn get_photo_by_name(name: &str, hash: &str, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM photos WHERE file_name = $1 AND file_path = $2")
            .await?;
        let result = client.query_one(&stmt, &[&name, &hash]).await?;

        let photo = Photo::from_row(result);

        Ok(photo)
    }

    pub async fn delete_photo(photo_id: i32, pool: &Pool) -> Result<String, PoolError> {
        let photo = Photo::get_by_id(photo_id, &pool).await?;

        // attempt to delete photo
        fs::remove_file(&photo.file_path).expect("Could not delete file");

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM photos WHERE id = $1").await?;
        let _result = client.execute(&stmt, &[&photo_id]).await?;

        Ok("File deleted successfully!".to_string())
    }

    pub async fn add_entity_to_photo(
        photo_id: i32,
        entity_id: i32,
        pool: &Pool,
    ) -> Result<String, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("insert into photo_entity (photo_id, entity_id) values ($1, $2)")
            .await?;
        let _ = client.execute(&stmt, &[&photo_id, &entity_id]).await?;

        let entity = Entity::get_by_id(entity_id, pool).await?;

        Ok(format!(
            "Entity `{}` added to photo successfully",
            entity.entity_name
        ))
    }

    pub async fn remove_entity_from_photo(
        photo_id: i32,
        entity_id: i32,
        pool: &Pool,
    ) -> Result<String, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("delete from photo_entity where photo_id = $1 and entity_id = $2")
            .await?;
        let _ = client.execute(&stmt, &[&photo_id, &entity_id]).await?;

        let entity = Entity::get_by_id(entity_id, pool).await?;

        Ok(format!(
            "Entity `{}` removed from photo successfully",
            entity.entity_name
        ))
    }

    pub async fn add_tag_to_photo(
        photo_id: i32,
        tag_id: i32,
        pool: &Pool,
    ) -> Result<String, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("insert into photo_tag (photo_id, tag_id) VALUES ($1, $2)")
            .await?;
        let _result = client.execute(&stmt, &[&photo_id, &tag_id]).await?;

        let tag = Tag::get_by_id(tag_id, pool).await?;

        Ok(format!(
            "Tag `{}` added to photo successfully",
            tag.tag_name
        ))
    }

    pub async fn remove_tag_from_photo(
        photo_id: i32,
        tag_id: i32,
        pool: &Pool,
    ) -> Result<String, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("delete from photo_tag where photo_id = $1 and tag_id = $2")
            .await?;
        let _result = client.execute(&stmt, &[&photo_id, &tag_id]).await?;

        let tag = Tag::get_by_id(tag_id, pool).await?;

        Ok(format!(
            "Tag `{}` removed from photo successfully",
            tag.tag_name
        ))
    }
}
