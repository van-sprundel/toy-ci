use std::sync::Arc;

use crate::app_state::AppState;
use crate::error::Result;
use crate::job::Job;
use crate::workspace_context::WorkspaceContext;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pipeline {
    pub trigger: Vec<String>,
    pub jobs: std::collections::HashMap<String, Job>,
}

impl Pipeline {
    pub fn should_trigger(&self, current_branch: &str) -> bool {
        self.trigger.contains(&current_branch.to_string())
    }

    pub async fn run(&self, state: &Arc<AppState>, context: &WorkspaceContext) -> Result<()> {
        for job in self.jobs.values() {
            job.run(state, context).await?;
        }

        Ok(())
    }
}
