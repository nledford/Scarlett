use crate::pagination::page::Page;
use crate::responses::api_response::ApiResponse;
use crate::schemas::photo_full::PhotoFull;

// API RESPONSES ***********************************************************************************

pub type PaginatedPhotoResponse = ApiResponse<PaginatedPhotos>;

// PAGINATION **************************************************************************************

pub type PaginatedPhotos = Page<Vec<PhotoFull>>;
