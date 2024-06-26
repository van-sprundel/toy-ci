use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::{app_state::AppState, build_executor::BuildExecutor, Result};

use super::build_message::BuildMessage;

pub struct Actor(mpsc::Receiver<BuildMessage>);

impl Actor {
    pub fn new(rx: mpsc::Receiver<BuildMessage>) -> Self {
        Self(rx)
    }

    pub async fn build_scheduler(&mut self, state: Arc<AppState>) {
        let executors: Vec<Arc<Mutex<BuildExecutor>>> = (0..5)
            .map(|_| Arc::new(Mutex::new(BuildExecutor::new())))
            .collect();

        while let Some(build_message) = self.0.recv().await {
            match build_message {
                BuildMessage::NewBuild(new_build_message) => {
                    let context = new_build_message.context;
                    let pipeline = new_build_message.pipeline;

                    'executor: loop {
                        for executor in &executors {
                            let mut executor = executor.lock().await;

                            if executor.is_available() {
                                tracing::debug!("Executor found for build_id: {}", &context.id);

                                let _ = executor
                                    .run_build(state.clone(), context.clone(), pipeline.clone())
                                    .await;

                                break 'executor;
                            }
                        }
                    }
                }
                BuildMessage::CancelBuild(cancel_build_message) => {
                    let build_id = cancel_build_message.0;
                    for executor in &executors {
                        let mut executor = executor.lock().await;
                        if executor.is_building(&build_id) {
                            tracing::info!("Cancelling build_id: {:?}", build_id);
                            let _ = executor.cancel().await;
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone)]
pub struct ActorHandler(Arc<Mutex<mpsc::Sender<BuildMessage>>>);

impl ActorHandler {
    pub fn new(tx: mpsc::Sender<BuildMessage>) -> Self {
        Self(Arc::new(Mutex::new(tx)))
    }

    pub async fn send(&self, build_message: BuildMessage) -> Result<()> {
        let tx = self.0.lock().await;
        tx.send(build_message).await?;
        Ok(())
    }
}
