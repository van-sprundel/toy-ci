#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubUser {
    date: String,
    email: Option<String>,
    name: String,
    username: String,
}
