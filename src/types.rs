use crate::models::responses::{ApiResponse, Page};
use crate::schemas::photo_full::PhotoFull;

// PAGINATION TYPES ********************************************************************************

pub type PaginatedPhotoResponse = ApiResponse<Page<PhotoFull>>;
