use super::{GithubCommit, GithubRepository, GithubUser};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubPushWebhookPayload {
    after: String,
    before: String,
    base_ref: Option<String>,
    pub commits: Vec<GithubCommit>,
    comprase: String,
    created: bool,
    deleted: bool,
    forced: bool,
    head_commit: Option<GithubCommit>,
    pusher: GithubUser,
    #[serde(rename = "ref")]
    reference: String,
    repository: GithubRepository,
    sender: GithubUser,
}
