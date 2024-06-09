use std::sync::Arc;

use tokio::sync::{mpsc, Mutex};

use crate::{build_executor::BuildExecutor, AppState};

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
            let executors = executors.clone();
            let state_clone = state.clone();
            let build_id = new_build_message.id.clone();
            let commit = new_build_message.commit.clone();

            tokio::spawn(async move {
                loop {
                    for executor in &executors {
                        let mut executor = executor.lock().await;
                        if executor.is_available() {
                            executor.run_build(state_clone, &build_id, commit).await;
                            return;
                        }
                    }
                }
            });
        }
    }
}

#[derive(Clone)]
pub struct ActorHandler(Arc<Mutex<mpsc::Sender<NewBuildMessage>>>);

impl ActorHandler {
    pub fn new(tx: mpsc::Sender<NewBuildMessage>) -> Self {
        Self(Arc::new(Mutex::new(tx)))
    }

    pub async fn send(
        &self,
        new_build_message: NewBuildMessage,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let tx = self.0.lock().await;
        tx.send(new_build_message).await?;
        Ok(())
    }
}
