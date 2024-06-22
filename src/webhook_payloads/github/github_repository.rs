#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubRepository {
    pub(crate) name: String,
    pub(crate) url: String,
}
