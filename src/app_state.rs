use std::collections::HashMap;

use futures::lock::Mutex;
use tokio::process::Command;

use crate::build_context::BuildContext;
use crate::running_build::RunningBuild;
use crate::Result;

#[derive(Default)]
pub struct AppState {
    builds: Mutex<HashMap<String, RunningBuild>>,
}

impl AppState {
    pub async fn send_log(&self, build_id: &str, message: &str) -> Result<()> {
        let mut build_progress_channel_map = self.builds.lock().await;

        if let Some(build) = build_progress_channel_map.get_mut(build_id) {
            let (tx, _) = &build.channel;
            tx.send(message.to_string())?;

            build.logs.push(message.to_string());
        }

        Ok(())
    }

    pub async fn create_build(&self, id: &str) {
        let (tx, rx) = tokio::sync::broadcast::channel(100);

        self.builds
            .lock()
            .await
            .insert(id.to_string(), RunningBuild::new((tx, rx)));
    }

    pub fn get_builds(&self) -> &Mutex<HashMap<String, RunningBuild>> {
        &self.builds
    }

    pub async fn create_git_directory_if_not_exists(&self, context: &BuildContext) -> Result<()> {
        let path = std::path::Path::new(&context.repo_dir);
        if path.exists() {
            return Ok(());
        }

        std::fs::create_dir_all(path)?;

        self.send_log(
            &context.id,
            &format!("Cloning {} into {}", context.repo_url, context.repo_dir),
        )
        .await?;

        let output = Command::new("git")
            .args(["clone", &context.repo_url, &context.repo_dir])
            .output()
            .await;

        match output {
            Ok(_) => Ok(()),
            Err(e) => Err(e.into()),
        }
    }
}
