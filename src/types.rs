use actix_web::{Error, HttpResponse};

use crate::errors::ServiceError;
use crate::pagination::page::Page;
use crate::responses::api_response::ApiResponse;
use crate::schemas::entity::Entity;
use crate::schemas::photo_full::PhotoFull;
use crate::schemas::tags::Tag;
use crate::stats::entities::EntityStats;
use crate::stats::tags::TagStats;

// API RESPONSES ***********************************************************************************

pub type PaginatedPhotoResponse = ApiResponse<PaginatedPhotos>;

// PAGINATION **************************************************************************************

pub type PaginatedPhotos = Page<Vec<PhotoFull>>;
pub type PaginatedEntities = Page<Vec<Entity>>;
pub type PaginatedEntityStats = Page<Vec<EntityStats>>;
pub type PaginatedTags = Page<Vec<Tag>>;
pub type PaginatedTagStats = Page<Vec<TagStats>>;

// RESULTS *****************************************************************************************

pub type DbSingleResult<T> = Result<T, ServiceError>;
pub type DbVecResult<T> = Result<Vec<T>, ServiceError>;
pub type DbMessageResult = Result<String, ServiceError>;
pub type HandlerResult = Result<HttpResponse, Error>;
