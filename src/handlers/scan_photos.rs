use std::path::Path;

use actix_web::{get, web};
use deadpool_postgres::Pool;
use num_format::{Locale, ToFormattedString};
use serde::{Deserialize, Serialize};

use crate::{files, schemas};
use crate::files::photos::FileScanResult;
use crate::responses::api_response::ApiResponse;
use crate::schemas::new_photo::NewPhoto;
use crate::types::HandlerResult;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanPhotosResult {
    pub folder_scanned: String,
    pub new_photos_found: bool,
    pub new_photos: i32,
    pub existing_photos: i32,
    pub updated_photos: i32,
    pub deleted_photos: i32,
}

impl Default for ScanPhotosResult {
    fn default() -> ScanPhotosResult {
        ScanPhotosResult {
            folder_scanned: String::default(),
            new_photos_found: false,
            new_photos: 0,
            existing_photos: 0,
            updated_photos: 0,
            deleted_photos: 0,
        }
    }
}

impl ScanPhotosResult {
    pub fn from_file_scan_result(folder: String, result: &FileScanResult) -> Self {
        ScanPhotosResult {
            folder_scanned: folder,
            new_photos_found: result.new_photos_count > 0,
            new_photos: result.new_photos_count,
            existing_photos: result.existing_photos_count,
            updated_photos: result.updated_photos_count,
            deleted_photos: result.deleted_photos_count,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ScanPhotosRequest {
    pub folder: Option<String>,
}

impl ScanPhotosRequest {
    pub fn get_folder(&self) -> String {
        self.folder
            .to_owned()
            .unwrap_or_else(|| String::from(""))
            .replace('\"', "")
    }
}

#[get("/scan")]
pub async fn run_scan(info: web::Query<ScanPhotosRequest>, pool: web::Data<Pool>) -> HandlerResult {
    let folder = info.get_folder();
    let pool = pool.get_ref();

    // Check if path exists before scanning
    if !Path::new(&folder).exists() {
        return Ok(ApiResponse::error(format!(
            "Directory not found: {}",
            folder
        )));
    }

    println!("Scanning {}...", &folder);

    let file_scan_result = if !folder.is_empty() {
        files::photos::scan_all_photos_from_dir(&folder, pool).await
    } else {
        files::photos::scan_all_photos(pool).await
    };

    if let Err(err) = file_scan_result {
        return Ok(ApiResponse::error(err.to_string()));
    }
    let file_scan_result = file_scan_result.unwrap();

    if file_scan_result.new_photos_count > 0 {
        println!(
            "Insert {} new photos into database...",
            &file_scan_result
                .new_photos_count
                .to_formatted_string(&Locale::en)
        );
        NewPhoto::bulk_insert(&file_scan_result.new_photos, pool).await?;
    }

    // refresh random order view
    println!("Refresh random seed...");
    schemas::reset_seed(&pool).await?;

    let result = ScanPhotosResult::from_file_scan_result(folder, &file_scan_result);

    println!("Done!");

    Ok(ApiResponse::success(result))
}
