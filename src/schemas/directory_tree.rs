use deadpool_postgres::Pool;
use serde_json::Value as JSON;
use tokio_postgres::Row;

use crate::types::DbSingleResult;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub path: String,
    pub show_children: bool,
    pub selected: bool,
    pub children: Option<Vec<Node>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryTree {
    pub tree: Node,
}

impl DirectoryTree {
    fn from_row(row: Row) -> Self {
        let json: JSON = row.get(0);
        serde_json::from_str(json.to_string().as_str()).unwrap()
    }
}

pub async fn get_directory_tree(pool: &Pool) -> DbSingleResult<DirectoryTree> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT * FROM directory_tree").await?;
    let result = client.query_one(&stmt, &[]).await?;

    let directory_tree = DirectoryTree::from_row(result);

    Ok(directory_tree)
}
