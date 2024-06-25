use crate::{container_method::ContainerMethod, job::Job};

#[derive(Debug, PartialEq)]
pub struct Pipeline {
    pub trigger_branches: Vec<String>,
    pub container_method: ContainerMethod,
    pub jobs: Vec<Job>,
}

impl Pipeline {
    pub fn should_trigger(&self, current_branch: &str) -> bool {
        self.trigger_branches.contains(&current_branch.to_string())
    }
}
