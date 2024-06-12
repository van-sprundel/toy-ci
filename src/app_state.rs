use std::collections::HashMap;

use futures::lock::Mutex;

use crate::running_build::RunningBuild;
use crate::Result;

#[derive(Default)]
pub struct AppState {
    pub builds: Mutex<HashMap<String, RunningBuild>>,
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
}
