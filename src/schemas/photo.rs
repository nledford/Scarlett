use std::fs;

use chrono::{NaiveDateTime, Utc};
use deadpool_postgres::Pool;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::schemas::entity::Entity;
use crate::schemas::tags::Tag;
use crate::schemas::wallpaper_sizes::WallpaperSize;
use crate::types::{DbMessageResult, DbSingleResult, DbVecResult};

#[derive(Serialize, Deserialize, Debug, Clone, PostgresMapper)]
#[pg_mapper(table = "photos")]
pub struct Photo {
    pub id: i32,
    pub file_path: String,
    pub file_name: String,
    pub file_hash: String,
    pub rating: i32,
    pub date_created: NaiveDateTime,
    pub date_updated: NaiveDateTime,
    pub last_viewed: Option<NaiveDateTime>,
    pub original_width: i32,
    pub original_height: i32,
    pub rotation: i32,
    pub ineligible_for_wallpaper: bool,
    pub anonymous_entities: bool,
}

impl Photo {
    pub async fn get_all(pool: &Pool) -> DbVecResult<Self> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos").await?;
        let results = client.query(&stmt, &[]).await?;
        let photos: Vec<Photo> = results
            .into_iter()
            .map(|result| Photo::from_row(result).unwrap())
            .collect();

        Ok(photos)
    }

    pub async fn get_by_id(photo_id: i32, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos WHERE id = $1").await?;
        let result = client.query_one(&stmt, &[&photo_id]).await?;

        let photo = Photo::from_row(result).unwrap();

        Ok(photo)
    }

    pub async fn update_photo(updated_photo: Photo, pool: &Pool) -> DbSingleResult<Self> {
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

    pub async fn get_photo_by_name(name: &str, hash: &str, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("SELECT * FROM photos WHERE file_name = $1 AND file_path = $2")
            .await?;
        let result = client.query_one(&stmt, &[&name, &hash]).await?;

        let photo = Photo::from_row(result).unwrap();

        Ok(photo)
    }

    pub async fn delete_photo(photo_id: i32, pool: &Pool) -> DbMessageResult {
        let photo = Photo::get_by_id(photo_id, &pool).await?;

        // attempt to delete photo
        fs::remove_file(&photo.file_path).expect("Could not delete file");

        let client = pool.get().await?;
        let stmt = client.prepare("DELETE FROM photos WHERE id = $1").await?;
        let _result = client.execute(&stmt, &[&photo_id]).await?;

        Ok("File deleted successfully!".to_string())
    }

    pub async fn update_last_viewed(photo_id: i32, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("UPDATE photos SET last_viewed = current_timestamp WHERE id = $1")
            .await?;
        let _ = client.execute(&stmt, &[&photo_id]).await?;

        let photo = Photo::get_by_id(photo_id, pool).await?;

        Ok(photo)
    }

    // ENTITIES ************************************************************************************

    pub async fn add_entity_to_photo(
        photo_id: i32,
        entity_id: i32,
        pool: &Pool,
    ) -> DbMessageResult {
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
    ) -> DbMessageResult {
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

    // TAGS ****************************************************************************************

    pub async fn add_tag_to_photo(photo_id: i32, tag_id: i32, pool: &Pool) -> DbMessageResult {
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

    pub async fn remove_tag_from_photo(photo_id: i32, tag_id: i32, pool: &Pool) -> DbMessageResult {
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

    // WALLPAPERS **********************************************************************************

    pub async fn add_wallpaper_to_photo(
        photo_id: i32,
        wallpaper_size_id: i32,
        file_path: String,
        pool: &Pool,
    ) -> DbMessageResult {
        let client = pool.get().await?;
        let stmt = client.prepare("insert into photo_wallpaper (photo_id, wallpaper_size_id, file_path) values ($1, $2, $3)").await?;
        let _ = client
            .execute(&stmt, &[&photo_id, &wallpaper_size_id, &file_path])
            .await?;

        let size = WallpaperSize::get_by_id(wallpaper_size_id, &pool).await?;

        Ok(format!(
            "Wallpaper size `{}` added to photo successfully",
            size.name
        ))
    }

    pub async fn remove_wallpaper_from_photo(
        photo_id: i32,
        wallpaper_size_id: i32,
        pool: &Pool,
    ) -> DbMessageResult {
        let client = pool.get().await?;
        let stmt = client
            .prepare("delete from photo_wallpaper where photo_id = $1 and wallpaper_size_id = $2")
            .await?;
        let _ = client
            .execute(&stmt, &[&photo_id, &wallpaper_size_id])
            .await?;

        let size = WallpaperSize::get_by_id(wallpaper_size_id, &pool).await?;

        Ok(format!(
            "Wallpaper size `{}` removed from photo successfully",
            size.name
        ))
    }
}
