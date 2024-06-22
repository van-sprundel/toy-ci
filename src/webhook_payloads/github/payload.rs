use super::{GithubCommit, GithubRepository, GithubUser};

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubPushWebhookPayload {
    after: String,
    before: String,
    base_ref: Option<String>,
    pub commits: Vec<GithubCommit>,
    compare: String,
    created: bool,
    deleted: bool,
    forced: bool,
    head_commit: Option<GithubCommit>,
    pusher: GithubUser,
    #[serde(rename = "ref")]
    pub(crate) reference: String, // ref/heads/main
    pub(crate) repository: GithubRepository,
    sender: GithubUser,
}
