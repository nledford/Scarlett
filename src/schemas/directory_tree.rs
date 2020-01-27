use deadpool_postgres::{Pool, PoolError};
use serde_json::Value as JSON;
use tokio_postgres::Row;

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Node {
    pub id: String,
    pub parent_id: String,
    pub name: String,
    pub children: Option<Vec<Node>>,
}

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectoryTree {
    pub tree: Node,
}

impl DirectoryTree {
    fn from_row(row: Row) -> DirectoryTree {
        let json: JSON = row.get(0);

        let directory_tree: DirectoryTree =
            serde_json::from_str(json.to_string().as_str()).unwrap();

        directory_tree
    }
}

pub async fn get_directory_tree(pool: &Pool) -> Result<DirectoryTree, PoolError> {
    let client = pool.get().await?;
    let stmt = client.prepare("SELECT * FROM directory_tree").await?;
    let result = client.query_one(&stmt, &[]).await?;

    let directory_tree = DirectoryTree::from_row(result);

    Ok(directory_tree)
}
