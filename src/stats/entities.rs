use deadpool_postgres::{Pool, PoolError};
use tokio_postgres::Row;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityStats {
    pub entity_name: String,
    pub photos_with_entity: i64,
    pub percentage_with_entity: f64,
    pub percentage_total: f64,
}

impl EntityStats {
    pub fn from_row(row: Row) -> Self {
        EntityStats {
            entity_name: row.get(0),
            photos_with_entity: row.get(1),
            percentage_with_entity: row.get(2),
            percentage_total: row.get(3),
        }
    }

    pub async fn get_entity_stats(pool: &Pool) -> Result<Vec<Self>, PoolError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from entity_stats").await?;
        let results = client.query(&stmt, &[]).await?;

        let stats = results.into_iter().map(EntityStats::from_row).collect();

        Ok(stats)
    }
}
