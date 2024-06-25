use crate::error::MerelError;
use crate::error::Result;
use std::sync::Arc;
use tokio::process::Command as TokioCommand;

use crate::app_state::AppState;

pub async fn run_command(
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
        state.send_log(build_id, &output_string).await;

        return Err(MerelError::CommandFailed(command.to_string(), output_string).into());
    }

    state.send_log(build_id, command).await;

    Ok(())
}
