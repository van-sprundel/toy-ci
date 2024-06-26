#[derive(Clone)]
pub struct WorkspaceContext {
    pub id: String,
    pub repo_dir: String,
    pub repo_url: String,
    pub commit_id: String,
}
