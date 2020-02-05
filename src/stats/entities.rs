use deadpool_postgres::Pool;
use rust_decimal::Decimal;
use tokio_postgres::Row;

use crate::errors::ServiceError;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EntityStats {
    pub entity_name: String,
    pub photos_with_entity: i64,
    pub percentage_with_entity: Decimal,
    pub percentage_total: Decimal,
}

impl EntityStats {
    pub fn from_row(row: Row) -> Self {
        EntityStats {
            entity_name: row.get(0),
            photos_with_entity: row.try_get(1).unwrap_or(0),
            percentage_with_entity: row.try_get(2).unwrap_or_default(),
            percentage_total: row.try_get(3).unwrap_or_default(),
        }
    }

    pub async fn get_entity_stats(pool: &Pool) -> Result<Vec<Self>, ServiceError> {
        let client = pool.get().await?;
        let stmt = client.prepare("select * from entity_stats").await?;
        let results = client.query(&stmt, &[]).await?;

        let stats = results.into_iter().map(EntityStats::from_row).collect();

        Ok(stats)
    }
}
