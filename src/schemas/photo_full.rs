use std::env;

use chrono::NaiveDateTime;
use deadpool_postgres::{Client, Pool, PoolError};
use percent_encoding::{AsciiSet, CONTROLS, percent_encode};
use serde::{Deserialize, Serialize};
use tokio_postgres::Row;
use tokio_postgres::types::ToSql;

use crate::pagination::links::Links;
use crate::pagination::page::Page;
use crate::pagination::page_metadata::PageMetadata;
use crate::requests::get_photos_request::GetPhotosRequest;
use crate::schemas::collections::Collection;
use crate::types::PaginatedPhotos;
use crate::utils::strings;

// TODO generate recommended wallpaper name
// `photos_all` view *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
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
    pub aspect_ratio: String,
    pub orientation: String,
    pub rotation: i32,
    pub ineligible_for_wallpaper: bool,
    pub anonymous_entities: bool,
    pub suggested_entity_name: String,
    pub wallpaper_file_name: String,
    pub entities: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub wallpapers: Option<Vec<String>>,

    pub media_url: String,
}

impl PhotoFull {
    pub fn from_row(row: Row) -> Self {
        let file_path: String = row.get(1);

        PhotoFull {
            id: row.get(0),
            file_path: file_path.clone(),
            folder: row.get(2),
            file_name: row.get(3),
            file_hash: row.get(4),
            rating: row.get(5),
            date_created: row.get(6),
            date_updated: row.get(7),
            original_width: row.get(8),
            original_height: row.get(9),
            aspect_ratio: row.get(10),
            orientation: row.get(11),
            rotation: row.get(12),
            ineligible_for_wallpaper: row.get(13),
            anonymous_entities: row.get(14),
            suggested_entity_name: row.get(15),
            wallpaper_file_name: row.get(16),
            entities: row.get(17),
            tags: row.get(18),
            wallpapers: row.get(19),

            media_url: PhotoFull::build_photo_url(file_path),
        }
    }

    fn from_paginated_row(row: Row) -> (Self, i64) {
        let file_path: String = row.get(1);

        let photo = PhotoFull {
            id: row.get(0),
            file_path: file_path.clone(),
            folder: row.get(2),
            file_name: row.get(3),
            file_hash: row.get(4),
            rating: row.get(5),
            date_created: row.get(6),
            date_updated: row.get(7),
            original_width: row.get(8),
            original_height: row.get(9),
            aspect_ratio: row.get(10),
            orientation: row.get(11),
            rotation: row.get(12),
            ineligible_for_wallpaper: row.get(13),
            anonymous_entities: row.get(14),
            suggested_entity_name: row.get(15),
            wallpaper_file_name: row.get(16),
            entities: row.get(17),
            tags: row.get(18),
            wallpapers: row.get(19),

            media_url: PhotoFull::build_photo_url(file_path),
        };

        let count = row.get(20);

        (photo, count)
    }

    pub async fn get_all(pool: &Pool) -> Result<Vec<PhotoFull>, PoolError> {
        let client: Client = pool.get().await?;
        let stmt = client.prepare("SELECT * FROM photos_all").await?;
        let rows = client.query(&stmt, &[]).await?;

        let photos = rows
            .into_iter()
            .map(PhotoFull::from_row)
            .collect::<Vec<PhotoFull>>();

        Ok(photos)
    }

    pub async fn get_by_id(id: i32, pool: &Pool) -> Result<Self, PoolError> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from photos_all where id = $1")
            .await?;
        let result = client.query_one(&stmt, &[&id]).await?;

        let photo = PhotoFull::from_row(result);

        Ok(photo)
    }

    pub async fn get_page(
        req: GetPhotosRequest,
        pool: &Pool,
    ) -> Result<PaginatedPhotos, PoolError> {
        let client = pool.get().await?;
        let mut params: Vec<&(dyn ToSql + Sync)> = vec![];

        // pre-emptive TODO: cleanup and optimize this procedurally built query
        let mut query = "select id,
                               file_path,
                               folder,
                               file_name,
                               file_hash,
                               rating,
                               date_created,
                               date_updated,
                               original_width,
                               original_height,
                               aspect_ratio,
                               orientation,
                               rotation,
                               ineligible_for_wallpaper,
                               anonymous_entities,
                               suggested_entity_name,
                               wallpaper_file_name,
                               entities,
                               tags,
                               wallpapers,
                               count(*) over ()
                        from (
                                 select row_number() over () as position, photos.*
                                 from (
                                          select pa.*
                                          from photos_all pa
                                                   inner join photo_ordering po
                                                              on pa.id = po.photo_id ".to_string();

        if req.has_collection_or_filters() {
            query += " \nWHERE ";

            // collections override custom filters since a collection should already have necessary filtering logic
            if req.collection_id.is_some() {
                let collection_id = &req.collection_id.unwrap();

                let collection = Collection::get_by_id(collection_id.to_owned(), pool).await?;

                query += format!(" ({}) ", collection.query).as_str();
            } else {
                // TODO add custom filter logic
            }
        }

        query += " order by po.position \n
              ) photos \n
     ) random";

        query += " WHERE random.position > $1 ";

        // sorting
        query += " ORDER BY ";

        if req.get_sort_by().is_some() {
            let sortings = PhotoFull::determine_sorting(req.clone().get_sort_by().unwrap());

            let length = sortings.len();
            for (index, (category, direction)) in sortings.into_iter().enumerate() {
                query += format!("{} {}", category, direction).as_str();

                if index >= length {
                    query += ", "
                }
            }
        } else {
            // aka random sorting
            query += " random.position "
        }

        query += " LIMIT $2";

        println!("\n{}\n", &query);

        let page_size = &req.get_page_size();
        let page = (req.get_page() - 1) * req.get_page_size();
        params.push(&page);
        params.push(page_size);

        let stmt = client.prepare(query.as_str()).await?;
        let rows = client.query(&stmt, params.as_slice()).await?;

        let results: Vec<(PhotoFull, i64)> = rows
            .into_iter()
            .map(PhotoFull::from_paginated_row)
            .collect();
        let total = results.get(0).map(|x| x.1).unwrap_or(0);
        let photos = results.into_iter().map(|x| x.0).collect();

        let metadata = PageMetadata::new(req.get_page(), req.get_page_size(), total);
        let links = Links::new(&req, total);
        let page = Page::new(metadata, links, photos);

        Ok(page)
    }

    fn build_photo_url(image_path: String) -> String {
        let divider = "photos";

        let hostname = env::var("SCARLETT_HOSTNAME")
            .expect("SCARLETT_HOSTNAME environment variable not found.");

        let divider_offset = &image_path
            .find(&divider)
            .unwrap_or_else(|| image_path.len());
        let divider_length = &divider.len();
        let index = divider_offset + divider_length + 1;

        let mut path = image_path;
        path.replace_range(..index, "");

        let url = format!("http://{}/media/{}", hostname, path);

        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'\'');
        let encoded = percent_encode(url.as_ref(), FRAGMENT);
        encoded.to_string()
    }

    fn determine_sorting(sorting: Vec<String>) -> Vec<(String, String)> {
        sorting
            .into_iter()
            .map(|item| {
                let contains_sort_order = strings::contains_sort_order(&item);

                let direction = if contains_sort_order {
                    let first_char = item.clone().chars().next().unwrap();

                    if first_char == '-' {
                        "DESC"
                    } else {
                        "ASC"
                    }
                } else {
                    "ASC"
                };

                let sort_by = strings::get_category_from_sort(&item).to_string();

                (sort_by, direction.to_string())
            })
            .collect::<Vec<(String, String)>>()
    }
}
