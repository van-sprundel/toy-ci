use crate::pipeline::Pipeline;

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
        _state: Arc<AppState>,
        context: WorkspaceContext,
        pipeline: Pipeline,
    ) -> Result<()> {
        tracing::info!("Building from new workspace {}", context.id);

        self.available.store(false, Ordering::SeqCst);

        let result = pipeline.run().await;

        tracing::info!("Finished building from workspace {}", context.id);

        self.available.store(true, Ordering::SeqCst);

        result
    }
}
