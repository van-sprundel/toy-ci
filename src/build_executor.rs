use crate::pipeline::{Pipeline, PipelineStatus};

use crate::app_state::AppState;
use crate::workspace_context::WorkspaceContext;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct BuildExecutor {
    available: AtomicBool,
}

impl BuildExecutor {
    pub fn new() -> Self {
        Self {
            available: AtomicBool::new(true),
        }
    }

    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }

    pub async fn run_build(
        &mut self,
        state: Arc<AppState>,
        context: WorkspaceContext,
        pipeline: Pipeline,
    ) -> Result<PipelineStatus> {
        tracing::info!("Building from new workspace {}", context.id);

        self.available.store(false, Ordering::SeqCst);

        let result = pipeline.run(&state, &context).await;

        tracing::info!("Finished building from workspace {}", context.id);
        tracing::debug!("Got result {:?}", &result);

        if let Some(running_build) = state.pop_build(&context.id).await {
            running_build.store_logs()?;
        }

        self.available.store(true, Ordering::SeqCst);

        Ok(result)
    }
}
