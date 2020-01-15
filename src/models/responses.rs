/*
 * SEE:
 *   - https://technick.net/guides/software/software_json_api_format/
 *   - https://jsonapi.org/
 */

use std::env;

use chrono::prelude::*;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use url::Url;

use crate::requests::get_photos_request::GetPhotosRequest;

const APP_NAME: &str = env!("CARGO_PKG_NAME");
const VERSION: &str = env!("CARGO_PKG_VERSION");

// STANDARD RESPONSE *******************************************************************************

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ApiResponse<T> {
    pub program: String,
    pub version: String,
    pub release: i64,
    pub datetime: NaiveDateTime,
    pub timestamp: i64,
    pub status: String,
    pub code: i64,
    pub message: String,
    pub data: T,
}

impl<T> ApiResponse<T> {
    pub fn new(status: &str, code: i64, message: &str, data: T) -> Self {
        ApiResponse {
            program: APP_NAME.to_string(),
            version: VERSION.to_string(),
            release: 0, // TODO get a release number
            datetime: Utc::now().naive_utc(),
            timestamp: Utc::now().timestamp_nanos(),
            status: status.to_string(),
            code,
            message: message.to_string(),
            data,
        }
    }
}

// PAGINATION **************************************************************************************

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Metadata {
    pub position: i64,
    pub page_size: i64,
    pub page_count: i64,
    pub total_items: i64,
}

impl Metadata {
    pub fn new(position: i64, page_size: i64, total_items: i64) -> Metadata {
        let page_count = (total_items as f64 / page_size as f64).ceil() as i64;

        Metadata {
            position,
            page_size,
            page_count,
            total_items,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Links {
    pub current: String,
    pub first: String,
    pub next: String,
    pub previous: String,
    pub last: String,
}

impl Links {
    /*pub fn new(req: &GetPhotosRequest, total_pages: i64) -> Links {
        let position = req.get_position();

        let position = if position <= 0 {
            0
        } else if page >= total_pages {
            total_pages
        } else {
            position
        };

        let next_page = if page >= total_pages {
            total_pages
        } else {
            page + 1
        };

        let previous_page = if page <= 0 { 1 } else { page - 1 };

        let (first_link, previous_link) = if page == 1 {
            ("".to_string(), "".to_string())
        } else {
            (build_link(1, req), build_link(previous_page, req))
        };

        let (next_link, last_link) = if page >= total_pages {
            ("".to_string(), "".to_string())
        } else {
            (build_link(next_page, req), build_link(total_pages, req))
        };

        let current_link = build_link(page, req);

        Links {
            current: current_link,
            first: first_link,
            previous: previous_link,
            next: next_link,
            last: last_link,
        }
    }*/

    pub fn default() -> Links {
        Links {
            current: String::from(""),
            first: String::from(""),
            next: String::from(""),
            previous: String::from(""),
            last: String::from(""),
        }
    }
}

fn build_link(page: i64, req: &GetPhotosRequest) -> String {
    let mut url = build_host_url();

    let page_size = req.get_page_size();
    //    let sort_by = &req.get_sort_by();
    //    let random_seed = req.get_random_seed();
    //    let folder = &req.get_folder();
    //    let to_delete = &req.get_to_delete().to_string();

    url.query_pairs_mut()
        .append_pair("page", format!("{}", page).as_str())
        .append_pair("page_size", format!("{}", page_size).as_str());
    //        .append_pair("sort_by", &sort_by.join(","))
    //        .append_pair("to_delete", to_delete);

    //    if req.is_random() {
    //        url.query_pairs_mut()
    //            .append_pair("random_seed", format!("{}", random_seed).as_str());
    //    }
    //
    //    if req.favorite.is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("favorite", &req.favorite.unwrap().to_string());
    //    }
    //
    //    if req.ineligible_wallpaper.is_some() {
    //        url.query_pairs_mut().append_pair(
    //            "ineligible_wallpaper",
    //            &req.ineligible_wallpaper.unwrap().to_string(),
    //        );
    //    }

    //    if req.hidden.is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("hidden", &req.hidden.unwrap().to_string());
    //    }
    //
    //    url.query_pairs_mut().append_pair("folder", folder);
    //
    //    if req.get_raw_ignore_folders().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_folders", &req.get_raw_ignore_folders().unwrap());
    //    }
    //
    //    if req.get_raw_tags().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("tags", &req.get_raw_tags().unwrap());
    //    }
    //
    //    if req.get_raw_ignore_tags().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_tags", &req.get_raw_ignore_tags().unwrap());
    //    }
    //
    //    if req.get_raw_people().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("people", &req.get_raw_people().unwrap());
    //    }
    //
    //    if req.get_raw_ignore_people().is_some() {
    //        url.query_pairs_mut()
    //            .append_pair("ignore_people", &req.get_raw_ignore_people().unwrap());
    //    }

    url.into_string()
}

fn build_host_url() -> Url {
    let host =
        env::var("SCARLETT_HOSTNAME").expect("SCARLETT_HOSTNAME environment variable not set");
    let url = Url::parse(format!("http://{}", host).as_str()).unwrap();

    url
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Page<T> {
    pub metadata: Metadata,
    pub links: Links,
    pub data: T,
}

impl<T> Page<T> {
    pub fn new(metadata: Metadata, links: Links, data: T) -> Page<T> {
        Page {
            metadata,
            links,
            data,
        }
    }
}
