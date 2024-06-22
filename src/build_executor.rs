use crate::app_state::AppState;
use crate::build_context::BuildContext;
use crate::Result;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tokio::process::Command as TokioCommand;

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
        tracing::info!("Building from new commit {}", context.id);
        let repo_url = context.repo_url;
        let repo_dir = context.repo_dir;
        let commit_id = context.commit_id;

        self.available.store(false, Ordering::SeqCst);

        let result = async {
            self.run_command(
                &state,
                &context.id,
                "git",
                Some(vec!["clone", &repo_url, &repo_dir]),
                None,
            )
            .await?;

            self.run_command(
                &state,
                &context.id,
                "git",
                Some(vec!["checkout", &commit_id]),
                Some(&repo_dir),
            )
            .await?;

            // pipeline steps

            // Simulate build time
            tokio::time::sleep(Duration::from_secs(10)).await;

            Ok(())
        }
        .await;

        self.available.store(true, Ordering::SeqCst);

        result
    }

    pub async fn run_command(
        &self,
        state: &Arc<AppState>,
        build_id: &str,
        command: &str,
        command_args: Option<Vec<&str>>,
        directory: Option<&str>,
    ) -> Result<()> {
        let mut c = TokioCommand::new(command);

        if let Some(args) = command_args {
            c.args(args);
        }

        if let Some(directory) = directory {
            c.current_dir(directory);
        }

        let output = c.output().await?;
        let output_string = String::from_utf8(output.stderr)?;

        if !output.status.success() {
            tracing::error!("Error: {:?}", &output_string);

            state.send_log(build_id, &output_string).await?;

            self.available.store(true, Ordering::SeqCst);
            return Err(
                merel::MerelError::CommandFailed(command.to_string(), output_string).into(),
            );
        }

        state.send_log(build_id, command).await?;

        Ok(())
    }
}
