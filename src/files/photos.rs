use std::fs;
use std::path::Path;
use std::time::SystemTime;

use deadpool_postgres::{Pool, PoolError};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::schemas::photo::{NewPhoto, Photo};

// FILE SCAN RESULT ********************************************************************************

pub struct FileScanResult {
    pub new_photos_count: i32,
    pub updated_photos_count: i32,
    pub deleted_photos_count: i32,
    pub new_photos: Vec<NewPhoto>,
}

impl Default for FileScanResult {
    fn default() -> Self {
        FileScanResult {
            new_photos_count: 0,
            updated_photos_count: 0,
            deleted_photos_count: 0,
            new_photos: Vec::new(),
        }
    }
}

// FILE INFO ***************************************************************************************

pub struct FileInfo {
    pub file_path: String,
    pub file_extension: String,
    pub date_created: SystemTime,
}

impl FileInfo {
    pub fn new_from_entry(entry: &DirEntry) -> Self {
        let ext = get_file_extension(&entry).to_lowercase();
        let metadata = entry.path().metadata().unwrap();
        let dt_created = metadata.created().unwrap_or_else(|_| SystemTime::now());

        FileInfo {
            file_path: entry.path().to_str().unwrap().to_string(),
            file_extension: ext,
            date_created: dt_created,
        }
    }
}

fn get_file_extension(entry: &DirEntry) -> String {
    let file_name_split: Vec<&str> = entry.file_name().to_str().unwrap().split('.').collect();

    // returns file extension
    (*file_name_split.last().unwrap()).to_string()
}

// SCAN FILES **************************************************************************************

pub async fn scan_all_photos(pool: &Pool) -> Result<FileScanResult, PoolError> {
    let photos_dir = "/photos";

    scan_all_photos_from_dir(photos_dir, pool).await
}

pub async fn scan_all_photos_from_dir(dir: &str, pool: &Pool) -> Result<FileScanResult, PoolError> {
    let mut result: FileScanResult = Default::default();

    let files = collect_files_from_directory(&dir, pool).await?;

    // build list of new photo candidates
    let mut photos: Vec<NewPhoto> = files
        .par_iter()
        .map(|f| NewPhoto::new(f.file_path.to_string(), f.date_created))
        .collect();

    // check if any photos have been moved
    let mut updated_photos: Vec<NewPhoto> = Vec::new();
    for new_photo in &photos {
        let name = &new_photo.file_name;
        let hash = &new_photo.file_hash;

        if check_if_photo_exists_by_file(name, hash, &pool).await? {
            // Photo exists, so we can retrieve it without checking for null
            let mut photo_to_update = Photo::get_photo_by_name(name, hash, pool).await?;

            // Update the file path of the existing photo with the file path of the "new" photo
            photo_to_update.file_path = new_photo.to_owned().file_path;

            // Save the changes to the database
            Photo::update_photo(photo_to_update, pool).await?;

            updated_photos.push(new_photo.to_owned())
        }
    }
    result.new_photos_count = photos.len() as i32;
    result.updated_photos_count = updated_photos.len() as i32;
    result.new_photos = photos.clone();

    // check if any photos have been deleted in directory
    result.deleted_photos_count = check_for_deleted_files_in_dir(&dir, pool).await? as i32;

    // If no photos have been updated, return early
    if updated_photos.is_empty() {
        return Ok(result);
    }

    // Remove updated photos from new photos collection
    for item in updated_photos {
        photos.retain(|p| p.file_name != item.file_name && p.file_hash != item.file_hash)
    }

    result.new_photos = photos;

    Ok(result)
}

async fn collect_files_from_directory(dir: &str, pool: &Pool) -> Result<Vec<FileInfo>, PoolError> {
    let mut files = Vec::new();

    let image_file_extensions = vec![
        "jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "webp", "pnm", "heic",
    ];

    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();

        let file_info = FileInfo::new_from_entry(&entry);

        // We don't care about directories, just the files inside them
        // so we'll skip them
        if is_dir(&entry) {
            continue;
        }

        // Skip the file if we don't consider it to be an image
        if !image_file_extensions.contains(&file_info.file_extension.as_str()) {
            continue;
        }

        // Check if the file is currently in the database
        if is_in_db(&file_info, &pool).await? {
            continue;
        }

        // Assume this is a new image
        files.push(file_info)
    }

    files.sort_by(|a, b| a.file_path.to_lowercase().cmp(&b.file_path.to_lowercase()));

    Ok(files)
}

fn is_dir(entry: &DirEntry) -> bool {
    let path = entry.path().to_str().unwrap();
    let md = fs::metadata(path).unwrap();
    md.is_dir()
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with('.'))
        .unwrap_or(false)
}

async fn is_in_db(file_info: &FileInfo, pool: &Pool) -> Result<bool, PoolError> {
    let client = pool.get().await?;
    let stmt = client
        .prepare(
            "select count(*)
                                                from photos
                                                where file_path = $1",
        )
        .await?;

    let result = client.query_one(&stmt, &[&file_info.file_path]).await?;
    let count: i64 = result.get(0);

    Ok(count > 0)
}

async fn check_if_photo_exists_by_file(
    name: &str,
    hash: &str,
    pool: &Pool,
) -> Result<bool, PoolError> {
    let client = pool.get().await?;
    let stmt = client.prepare("select count(file_hash)
                                                                from photos
                                                                where file_hash = $1 and file_name = $2").await?;
    let result = client.query_one(&stmt, &[&hash, &name]).await?;
    let count: i64 = result.get(0);

    Ok(count > 0)
}

async fn check_for_deleted_files_in_dir(dir: &str, pool: &Pool) -> Result<usize, PoolError> {
    let mut deleted_files: Vec<String> = Vec::new();

    let client = pool.get().await?;

    // get file paths
    let file_paths_stmt = client
        .prepare("SELECT file_path FROM photos WHERE file_path LIKE '%' || $1 || '%'")
        .await?;
    let results = client.query(&file_paths_stmt, &[&dir]).await?;
    let file_paths = results
        .into_iter()
        .map(|row| row.get(0))
        .collect::<Vec<String>>();

    // iterate through files and check if any have been deleted
    let stmt = client
        .prepare("DELETE FROM photos WHERE file_path = $1")
        .await?;
    for file in &file_paths {
        let exists = Path::new(file).exists();

        if !exists {
            deleted_files.push(file.to_owned());

            // Remove file from database
            client.execute(&stmt, &[&file]).await?;
        }
    }

    Ok(deleted_files.len())
}
