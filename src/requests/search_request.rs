
#[derive(Debug, serde::Deserialize)]
pub struct SearchRequest {
    pub q: String,
}