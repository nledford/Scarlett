use deadpool_postgres::{Pool, PoolError};
use rust_decimal::Decimal;
use tokio_postgres::Row;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagStats {
    pub tag_name: String,
    pub photos_with_tag: i64,
    pub percentage_with_tag: Decimal,
    pub percentage_total: Decimal,
}

impl TagStats {
    fn from_row(row: Row) -> Self {
        TagStats {
            tag_name: row.get(0),
            photos_with_tag: row.try_get(1).unwrap_or(0),
            percentage_with_tag: row.try_get(2).unwrap_or_default(),
            percentage_total: row.try_get(3).unwrap_or_default(),
        }
    }

    pub async fn get_tag_stats(pool: &Pool) -> Result<Vec<Self>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from tag_stats").await?;
        let results = client.query(&stmt, &[]).await?;

        let stats = results.into_iter().map(TagStats::from_row).collect();

        Ok(stats)
    }
}
