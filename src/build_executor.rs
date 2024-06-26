use crate::pipeline::{Pipeline, PipelineStatus};

use crate::app_state::AppState;
use crate::workspace_context::WorkspaceContext;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

pub struct BuildExecutor {
    current_build_id: Option<String>,
    available: AtomicBool,
    cancel_flag: Arc<AtomicBool>,
}

impl BuildExecutor {
    pub fn new() -> Self {
        Self {
            available: AtomicBool::new(true),
            current_build_id: None,
            cancel_flag: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }

    pub fn is_building(&self, build_id: &str) -> bool {
        self.current_build_id.as_deref() == Some(build_id)
    }

    pub async fn run_build(
        &mut self,
        state: Arc<AppState>,
        context: WorkspaceContext,
        pipeline: Pipeline,
    ) -> Result<PipelineStatus> {
        tracing::info!("Building from new workspace {}", &context.id);
        self.current_build_id = Some(context.id.clone());
        self.available.store(false, Ordering::SeqCst);
        self.cancel_flag.store(false, Ordering::SeqCst);

        let result = pipeline.run(&state, &context, self.cancel_flag.clone()).await;

        tracing::info!("Finished building from workspace {}", context.id);
        tracing::debug!("Got result {:?}", &result);

        if let Some(running_build) = state.pop_build(&context.id).await {
            running_build.store_logs()?;
        }

        self.reset();

        Ok(result)
    }

    pub async fn cancel(&mut self) -> Result<()> {
        self.reset();

        Ok(())
    }

    fn reset(&mut self) {
        self.cancel_flag.store(true, Ordering::SeqCst);
        self.available.store(true, Ordering::SeqCst);
        self.current_build_id = None;
    }
}
