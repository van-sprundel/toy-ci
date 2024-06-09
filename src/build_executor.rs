use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::fs;
use tokio::process::Command as TokioCommand;

use crate::git::commit::Commit;
use crate::AppState;

#[derive(Clone)]
pub struct BuildExecutor {
    available: Arc<AtomicBool>,
}

impl BuildExecutor {
    pub fn new() -> Self {
        Self {
            available: Arc::new(AtomicBool::new(true)),
        }
    }

    pub fn is_available(&self) -> bool {
        self.available.load(Ordering::SeqCst)
    }

    pub async fn run_build(&mut self, state: Arc<AppState>, build_id: &str, commit: Commit) {
        tracing::info!("Building from new commit {build_id}");

        self.available.store(false, Ordering::SeqCst);

        let repo_url = &commit.url;
        let commit_id = &commit.id;
        let repo_dir = format!("/tmp/repo-{}", commit_id);

        fs::create_dir_all(&repo_dir).await.unwrap();
        let _ = state.send_log(build_id, "Created repo directory").await;

        TokioCommand::new("git")
            .args(["clone", repo_url, &repo_dir])
            .output()
            .await
            .unwrap();
        let _ = state.send_log(build_id, "Cloned repository").await;

        TokioCommand::new("git")
            .args(["checkout", commit_id])
            .current_dir(&repo_dir)
            .output()
            .await
            .unwrap();
        let _ = state.send_log(build_id, "Checked out commit").await;

        tokio::time::sleep(Duration::from_secs(10)).await;
        let _ = state.send_log(build_id, "Done!").await;

        // pipeline steps

        // Send result to notification system

        self.available.store(true, Ordering::SeqCst);
    }
}
