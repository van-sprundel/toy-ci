use crate::Result;
use tokio::sync::broadcast;

const LOGS_DIR: &'static str = "/tmp/merel/logs";

type Channel = (broadcast::Sender<String>, broadcast::Receiver<String>);

pub struct RunningBuild {
    pub id: String,
    pub channel: Channel,
    pub logs: Vec<String>,
}

impl RunningBuild {
    pub fn new(id: &str, channel: Channel) -> Self {
        Self {
            id: id.to_string(),
            channel,
            logs: vec![],
        }
    }

    pub fn store_logs(&self) -> Result<()> {
        std::fs::create_dir_all(LOGS_DIR)?;

        let mut logs = self.logs.join("\n");
        logs += "\n";

        tracing::trace!("{logs}");

        let logs_path = format!("{}/{}-logs.txt", LOGS_DIR, self.id);
        tracing::debug!("Storing logs at {}", &logs_path);
        std::fs::write(logs_path, logs)?;

        Ok(())
    }
}
