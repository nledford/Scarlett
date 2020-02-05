use actix_web::{get, web};
use deadpool_postgres::Pool;

use crate::responses::api_response::ApiResponse;
use crate::schemas::wallpaper_sizes::WallpaperSize;
use crate::types::HandlerResult;

// ALL WALLPAPER SIZES *****************************************************************************

#[get("/wallpaper_sizes")]
pub async fn get_wallpaper_sizes(pool: web::Data<Pool>) -> HandlerResult {
    let sizes = WallpaperSize::get_all(&pool).await?;

    Ok(ApiResponse::success(sizes))
}
