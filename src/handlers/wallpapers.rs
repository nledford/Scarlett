use actix_web::{get, HttpResponse, web};
use deadpool_postgres::Pool;

use crate::errors::ServiceError;
use crate::responses::api_response::ApiResponse;
use crate::schemas::wallpaper_sizes::WallpaperSize;

// ALL WALLPAPER SIZES *****************************************************************************

#[get("/wallpaper_sizes")]
pub async fn get_wallpaper_sizes(pool: web::Data<Pool>) -> Result<HttpResponse, ServiceError> {
    let sizes = WallpaperSize::get_all(&pool).await?;

    Ok(ApiResponse::success(sizes))
}
