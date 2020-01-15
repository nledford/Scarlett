use crate::models::responses::ApiResponse;
use crate::pagination::page::Page;
use crate::schemas::photo_full::PhotoFull;

// API RESPONSES ***********************************************************************************

pub type PaginatedPhotoResponse = ApiResponse<PaginatedPhotos>;

// PAGINATION **************************************************************************************

pub type PaginatedPhotos = Page<Vec<PhotoFull>>;
