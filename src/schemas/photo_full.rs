use chrono::NaiveDateTime;
use deadpool_postgres::{Client, Pool, PoolError};
use serde::{Deserialize, Serialize};
use tokio_postgres::types::ToSql;
use tokio_postgres::Row;

use crate::pagination::links::Links;
use crate::pagination::page::Page;
use crate::pagination::page_metadata::PageMetadata;
use crate::requests::get_photos_request::GetPhotosRequest;
use crate::types::PaginatedPhotos;

// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PhotoFull {
    pub id: i32,
    pub file_path: String,
    pub folder: String,
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
    pub entities: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub wallpapers: Option<Vec<String>>,
}

impl PhotoFull {
    pub fn from_row(row: Row) -> Self {
        PhotoFull {
            id: row.get(0),
            file_path: row.get(1),
            folder: row.get(2),
            file_name: row.get(3),
            file_hash: row.get(4),
            rating: row.get(5),
            date_created: row.get(6),
            date_updated: row.get(7),
            original_width: row.get(8),
            original_height: row.get(9),
            rotation: row.get(10),
            ineligible_for_wallpaper: row.get(11),
            anonymous_entities: row.get(12),
            entities: row.get(13),
            tags: row.get(14),
            wallpapers: row.get(15),
        }
    }

    pub fn from_paginated_row(row: Row) -> (Self, i64) {
        let photo = PhotoFull {
            id: row.get(0),
            file_path: row.get(1),
            folder: row.get(2),
            file_name: row.get(3),
            file_hash: row.get(4),
            rating: row.get(5),
            date_created: row.get(6),
            date_updated: row.get(7),
            original_width: row.get(8),
            original_height: row.get(9),
            rotation: row.get(10),
            ineligible_for_wallpaper: row.get(11),
            anonymous_entities: row.get(12),
            entities: row.get(13),
            tags: row.get(14),
            wallpapers: row.get(15),
        };

        let count = row.get(16);

        (photo, count)
    }

    pub async fn all_photos(pool: &Pool) -> Result<Vec<PhotoFull>, PoolError> {
        let client: Client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos_all").await?;
        let rows = client.query(&stmt, &[]).await?;

        let photos = rows
            .into_iter()
            .map(PhotoFull::from_row)
            .collect::<Vec<PhotoFull>>();

        Ok(photos)
    }

    pub async fn get_page(
        req: GetPhotosRequest,
        pool: &Pool,
    ) -> Result<PaginatedPhotos, PoolError> {
        let client = pool.get().await?;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];

        // pre-emptive TODO: cleanup and optimize this procedurally built query
        let mut query = "SELECT    id,
                                          file_path,
                                          folder,
                                          file_name,
                                          file_hash,
                                          rating,
                                          date_created,
                                          date_updated,
                                          original_width,
                                          original_height,
                                          rotation,
                                          ineligible_for_wallpaper,
                                          anonymous_entities,
                                          entities,
                                          tags,
                                          wallpapers, \
                                          COUNT(*) OVER () \
                                   FROM (SELECT row_number() OVER () as position, pa.* \
                                         FROM photos_all pa \
                                                   INNER JOIN photo_ordering po ON pa.id = po.photo_id".to_string();

        query += "       ORDER BY po.position) t \
                  WHERE t.position > $1 \
                  ORDER BY t.position \
                  LIMIT $2";

        let page_size = &req.get_page_size();
        let page = &req.get_position();
        params.push(page);
        params.push(page_size);

        let stmt = client.prepare(query.as_str()).await?;
        let rows = client.query(&stmt, params.as_slice()).await?;

        let results: Vec<(PhotoFull, i64)> = rows
            .into_iter()
            .map(PhotoFull::from_paginated_row)
            .collect();
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let photos = results.into_iter().map(|x| x.0).collect();

        let metadata = PageMetadata::new(req.get_position(), req.get_page_size(), total);
        let links = Links::default();
        let page = Page::new(metadata, links, photos);

        Ok(page)
    }
}
