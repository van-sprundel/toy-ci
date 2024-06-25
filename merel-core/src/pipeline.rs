use crate::job::Job;

#[derive(Default)]
pub struct Pipeline {
    pub trigger_branch: String,
    pub jobs: Vec<Job>,
}

impl Pipeline {
    pub fn should_trigger(&self, current_branch: &str) -> bool {
        self.trigger_branch == current_branch
    }
}
