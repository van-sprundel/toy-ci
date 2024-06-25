use crate::app_state::AppState;
use crate::build_context::BuildContext;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

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

    pub async fn run_build(&mut self, state: Arc<AppState>, context: BuildContext) -> Result<()> {
        tracing::info!("Building from new workspace {}", context.id);

        self.available.store(false, Ordering::SeqCst);

        let result = async {
            // pipeline steps

            // Simulate build time
            tokio::time::sleep(Duration::from_secs(10)).await;

            Ok(())
        }
        .await;

        tracing::info!("Finished building from workspace {}", context.id);

        self.available.store(true, Ordering::SeqCst);

        result
    }
}
