#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct Commit {
    pub id: String,
    pub url: String,
}
