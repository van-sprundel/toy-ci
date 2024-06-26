use std::sync::Arc;

use crate::app_state::AppState;
use crate::step::Step;
use crate::workspace_context::WorkspaceContext;
use crate::Result;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Job {
    image: String,
    steps: Vec<Step>,
}

impl Job {
    pub async fn run(&self, state: &Arc<AppState>, context: &WorkspaceContext) -> Result<()> {
        for step in &self.steps {
            state.send_log(&context.id, step).await;
            // run step
        }

        Ok(())
    }
}
