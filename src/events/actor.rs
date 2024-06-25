use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::{app_state::AppState, build_executor::BuildExecutor, Result};

use super::new_build_message::NewBuildMessage;

pub struct Actor(mpsc::Receiver<NewBuildMessage>);

impl Actor {
    pub fn new(rx: mpsc::Receiver<NewBuildMessage>) -> Self {
        Self(rx)
    }

    pub async fn build_scheduler(&mut self, state: Arc<AppState>) {
        let executors: Vec<Arc<Mutex<BuildExecutor>>> = (0..5)
            .map(|_| Arc::new(Mutex::new(BuildExecutor::new())))
            .collect();

        while let Some(new_build_message) = self.0.recv().await {
            let context = new_build_message.context;

            'executor: loop {
                for executor in &executors {
                    let mut executor = executor.lock().await;

                    if executor.is_available() {
                        tracing::debug!("Executor found for build_id: {}", &context.id);

                        // we ignore errors here so it can safely stop the loop
                        let _ = executor.run_build(state.clone(), context.clone()).await;

                        break 'executor;
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ActorHandler(Arc<Mutex<mpsc::Sender<NewBuildMessage>>>);

impl ActorHandler {
    pub fn new(tx: mpsc::Sender<NewBuildMessage>) -> Self {
        Self(Arc::new(Mutex::new(tx)))
    }

    pub async fn send(&self, new_build_message: NewBuildMessage) -> Result<()> {
        let tx = self.0.lock().await;
        tx.send(new_build_message).await?;
        Ok(())
    }
}
