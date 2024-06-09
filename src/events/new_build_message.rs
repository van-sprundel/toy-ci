use crate::git::commit::Commit;

#[derive(Clone)]
pub struct NewBuildMessage {
    pub commit: Commit,
    pub id: String,
}
