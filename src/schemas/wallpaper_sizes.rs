use deadpool_postgres::Pool;
use tokio_pg_mapper::FromTokioPostgresRow;
use tokio_pg_mapper_derive::PostgresMapper;

use crate::types::{DbVecResult, DbMessageResult, DbSingleResult};

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug, PostgresMapper)]
#[pg_mapper(table = "wallpaper_sizes")]
pub struct WallpaperSize {
    pub id: i32,
    pub name: String,
    pub width: i32,
    pub height: i32,
}

impl WallpaperSize {
    pub async fn get_all(pool: &Pool) -> DbVecResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from wallpaper_sizes order by (width, height)")
            .await?;
        let results = client.query(&stmt, &[]).await?;

        let sizes: Vec<WallpaperSize> = results
            .into_iter()
            .map(|result| WallpaperSize::from_row(result).unwrap())
            .collect();

        Ok(sizes)
    }

    pub async fn get_by_id(id: i32, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from wallpaper_sizes where id = $1")
            .await?;
        let result = client.query_one(&stmt, &[&id]).await?;
        let size = WallpaperSize::from_row(result).unwrap();

        Ok(size)
    }

    pub async fn create(
        name: &str,
        width: i32,
        height: i32,
        pool: &Pool,
    ) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("insert into wallpaper_sizes (name, width, height) values ($1, $2, $3)")
            .await?;
        let _ = client.execute(&stmt, &[&name, &width, &height]).await?;

        let collection = WallpaperSize::get_by_name(name, pool).await?;

        Ok(collection)
    }

    pub async fn update(size: WallpaperSize, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;

        let stmt = client
            .prepare(
                "update wallpaper_sizes \
                 set name = $1, width = $2, height = $3 \
                 where id = $4",
            )
            .await?;

        let _ = client
            .execute(&stmt, &[&size.name, &size.width, &size.height, &size.id])
            .await?;

        let result = WallpaperSize::get_by_id(size.id, pool).await?;

        Ok(result)
    }

    pub async fn delete(id: i32, pool: &Pool) -> DbMessageResult {
        let collection = WallpaperSize::get_by_id(id, pool).await?;

        let client = pool.get().await?;
        let stmt = client
            .prepare("delete from wallpaper_sizes where id = $1")
            .await?;
        let _ = client.execute(&stmt, &[&collection.id]).await?;

        Ok("Wallpaper Size deleted successfully".to_string())
    }

    pub async fn get_by_name(name: &str, pool: &Pool) -> DbSingleResult<Self> {
        let client = pool.get().await?;
        let stmt = client
            .prepare("select * from wallpaper_sizes where name = $1")
            .await?;
        let result = client.query_one(&stmt, &[&name]).await?;
        let collection = WallpaperSize::from_row(result).unwrap();

        Ok(collection)
    }
}
