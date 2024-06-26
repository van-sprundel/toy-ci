use std::sync::Arc;

use crate::app_state::AppState;
use crate::job::Job;
use crate::workspace_context::WorkspaceContext;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Pipeline {
    pub trigger: Vec<String>,
    pub jobs: std::collections::HashMap<String, Job>,
}

#[derive(Debug)]
pub enum PipelineStatus {
    Passed,
    Cancelled,
    Failed,
}

impl Pipeline {
    pub fn should_trigger(&self, current_branch: &str) -> bool {
        self.trigger.contains(&current_branch.to_string())
    }

    pub async fn run(&self, state: &Arc<AppState>, context: &WorkspaceContext) -> PipelineStatus {
        for job in self.jobs.values() {
            if let Err(e) = job.run(state, context).await {
                let error_message = format!("Pipeline failed with error:\n {}", &e.to_string());
                state.send_log(&context.id, &error_message).await;
                return PipelineStatus::Failed;
            }
        }

        PipelineStatus::Passed
    }
}
