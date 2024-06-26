use std::sync::Arc;

use crate::app_state::AppState;
use crate::prelude::*;
use crate::step::Step;
use crate::workspace_context::BuildContext;
use crate::Result;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Job {
    image: String,
    steps: Vec<Step>,
}

impl Job {
    pub async fn run(&self, state: &Arc<AppState>, context: &BuildContext) -> Result<()> {
        for step in &self.steps {
            let command_log = format!("run: {step}");
            state.send_log(&context.id, &command_log).await;

            let output = Command::new("sh")
                .args(["-c", step])
                .current_dir(&context.repo_dir)
                .output()
                .await?;

            let output = String::from_utf8(output.stdout)?;
            tracing::trace!("Got output {output}");
            state.send_log(&context.id, &output).await;
        }

        Ok(())
    }
}
