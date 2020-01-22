use tokio_postgres::Row;
use deadpool_postgres::{Pool, PoolError};


#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagStats {
    pub tag_name: String,
    pub photos_with_tag: i64,
    pub percentage_with_tag: f64,
    pub percentage_total: f64,
}

impl TagStats {
    fn from_row(row: Row) -> Self {
        TagStats {
            tag_name: row.get(0),
            photos_with_tag: row.get(1),
            percentage_with_tag: row.get(2),
            percentage_total: row.get(3),
        }
    }

    pub async fn get_tag_stats(pool: &Pool) -> Result<TagStats, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from tag_stats").await?;
        let result = client.query_one(&stmt, &[]).await?;

        let stats = TagStats::from_row(result);

        Ok(stats)
    }
}
