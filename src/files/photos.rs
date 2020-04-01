use std::path::Path;
use std::time::SystemTime;

use deadpool_postgres::{Pool, PoolError};
use num_format::{Locale, ToFormattedString};
use rayon::prelude::*;
use walkdir::{DirEntry, WalkDir};

use crate::errors::ServiceError;
use crate::errors::ServiceError::DuplicateFileError;
use crate::schemas::new_photo::NewPhoto;
use crate::schemas::photo::Photo;
use crate::types::FileCollectionResult;

// FILE SCAN RESULT ********************************************************************************

#[derive(Clone)]
pub struct FileScanResult {
    pub existing_photos_count: i32,
    pub new_photos_count: i32,
    pub updated_photos_count: i32,
    pub deleted_photos_count: i32,
    pub new_photos: Vec<NewPhoto>,
}

impl Default for FileScanResult {
    fn default() -> Self {
        FileScanResult {
            existing_photos_count: 0,
            new_photos_count: 0,
            updated_photos_count: 0,
            deleted_photos_count: 0,
            new_photos: Vec::new(),
        }
    }
}

// DUPLICATE FILES RESULT **************************************************************************

#[derive(Debug, serde::Serialize)]
pub struct DuplicatePhoto {
    pub file_hash: String,
    pub files: Vec<String>,
}

impl Default for DuplicatePhoto {
    fn default() -> Self {
        DuplicatePhoto {
            file_hash: String::default(),
            files: Vec::default(),
        }
    }
}

impl DuplicatePhoto {
    pub fn new(file_hash: &str, files: Vec<String>) -> Self {
        DuplicatePhoto {
            file_hash: file_hash.to_string(),
            files,
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

pub async fn scan_all_photos(pool: &Pool) -> Result<FileScanResult, ServiceError> {
    let photos_dir = "/photos";

    scan_all_photos_from_dir(photos_dir, pool).await
}

pub async fn scan_all_photos_from_dir(
    dir: &str,
    pool: &Pool,
) -> Result<FileScanResult, ServiceError> {
    println!("Collecting files...");
    let (files, existing_files_count) = collect_files_from_directory(&dir, pool).await?;

    let mut result: FileScanResult = Default::default();
    result.existing_photos_count = existing_files_count;

    println!(
        "Found {} new files. ({} already exist)",
        &files.len().to_formatted_string(&Locale::en),
        &existing_files_count.to_formatted_string(&Locale::en),
    );

    println!("Build list of new photo candidates...");
    // build list of new photo candidates
    let mut photos: Vec<NewPhoto> = files
        .par_iter()
        .map(|f| NewPhoto::new(f.file_path.to_string(), f.date_created))
        .collect();

    println!("Check for duplicate photos...");
    let duplicate_photos: Vec<DuplicatePhoto> = check_for_duplicates(&photos, pool).await?;

    if !duplicate_photos.is_empty() {
        println!("Number of duplicates: {}", &duplicate_photos.len());
        for dup in &duplicate_photos {
            println!("File Hash: {}", dup.file_hash);

            println!("Files:");
            for file in &dup.files {
                println!("{}", file);
            }
        }
        return Err(DuplicateFileError(duplicate_photos));
    }
    println!("No duplicate photos found.");

    println!("Check for moved photos...");
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

    println!("Delete photos if necessary...");
    // check if any photos have been deleted in directory
    result.deleted_photos_count = check_for_deleted_files_in_dir(&dir, pool).await? as i32;

    // If no photos have been updated, return early
    if updated_photos.is_empty() {
        return Ok(result);
    }

    println!("Remove updated photos from new photos collection...");
    // Remove updated photos from new photos collection
    for item in updated_photos {
        photos.retain(|p| p.file_name != item.file_name && p.file_hash != item.file_hash)
    }

    result.new_photos = photos;

    Ok(result)
}

async fn check_for_duplicates(
    new_photos: &[NewPhoto],
    pool: &Pool,
) -> Result<Vec<DuplicatePhoto>, ServiceError> {
    let mut duplicate_photos = Vec::new();

    for new_photo in new_photos {
        let hash = &new_photo.file_hash;
        let file_path = &new_photo.file_path;

        let mut duplicates = check_for_duplicate(hash, pool).await?;

        if duplicates.is_empty() {
            continue;
        }

        // check if both duplicates haven't been deleted
        for duplicate in duplicates {
            let exists = Path::new(&duplicate).exists();

            if !exists {
                Photo::delete_photo_by_path(&duplicate, pool).await?;
            }
        }

        // check again for duplicates
        duplicates = check_for_duplicate(hash, pool).await?;

        if duplicates.is_empty() {
            continue;
        }

        // Duplicates exists

        if !duplicates.contains(file_path) {
            duplicates.push(file_path.to_string());
        }
        duplicates.sort();

        let duplicate = DuplicatePhoto::new(hash, duplicates);

        duplicate_photos.push(duplicate);
    }

    Ok(duplicate_photos)
}

async fn check_for_duplicate(hash: &str, pool: &Pool) -> Result<Vec<String>, ServiceError> {
    let client = pool.get().await?;
    let stmt = client
        .prepare("select file_path from photos where file_hash = $1")
        .await?;
    let results = client.query(&stmt, &[&hash]).await?;

    let mut paths = Vec::new();

    if results.is_empty() {
        return Ok(paths);
    }

    paths = results.into_iter().map(|row| row.get(0)).collect();

    Ok(paths)
}

async fn collect_files_from_directory(dir: &str, pool: &Pool) -> FileCollectionResult {
    let mut files = Vec::new();

    let image_file_extensions = vec![
        "jpg", "jpeg", "png", "gif", "bmp", "ico", "tiff", "webp", "pnm", "heic",
    ];

    let mut existing_files = 0;
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        let entry = entry.unwrap();
        let metadata = entry.metadata().unwrap();

        // We don't care about directories, just the files inside them
        // so we'll skip them
        if metadata.is_dir() {
            continue;
        }

        let file_info = FileInfo::new_from_entry(&entry);

        // Skip the file if we don't consider it to be an image
        if !image_file_extensions.contains(&file_info.file_extension.as_str()) {
            continue;
        }

        // Check if the file is currently in the database
        // if yes, increment counter
        if is_in_db(&file_info, &pool).await? {
            existing_files += 1;
            continue;
        }

        // Assume this is a new image
        files.push(file_info)
    }

    files.sort_by(|a, b| a.file_path.to_lowercase().cmp(&b.file_path.to_lowercase()));

    Ok((files, existing_files))
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
) -> Result<bool, ServiceError> {
    let client = pool.get().await?;
    let stmt = client.prepare("select count(file_hash)
                                                                from photos
                                                                where file_hash = $1 and file_name = $2").await?;
    let result = client.query_one(&stmt, &[&hash, &name]).await?;
    let count: i64 = result.get(0);

    Ok(count > 0)
}

async fn check_for_deleted_files_in_dir(dir: &str, pool: &Pool) -> Result<usize, ServiceError> {
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
    for file in &file_paths {
        let exists = Path::new(file).exists();

        if !exists {
            deleted_files.push(file.to_owned());

            // Remove file from database
            Photo::delete_photo_by_path(file, pool).await?;
        }
    }

    Ok(deleted_files.len())
}
