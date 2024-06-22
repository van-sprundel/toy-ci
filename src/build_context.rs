#[derive(Clone)]
pub struct BuildContext {
    pub id: String,
    pub repo_dir: String,
    pub repo_url: String,
    pub commit_id: String,
}
