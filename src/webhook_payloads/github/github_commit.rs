use crate::git::commit::Commit;

use super::GithubUser;

#[derive(serde::Deserialize, serde::Serialize, Clone)]
pub struct GithubCommit {
    added: Vec<String>,
    author: GithubUser,
    committer: GithubUser,
    distinct: bool,
    pub id: String,
    message: String,
    modified: Vec<String>,
    removed: Vec<String>,
    timestamp: String,
    tree_id: String,
    pub url: String,
}

impl From<GithubCommit> for Commit {
    fn from(val: GithubCommit) -> Self {
        Commit {
            id: val.id,
            url: val.url,
        }
    }
}
