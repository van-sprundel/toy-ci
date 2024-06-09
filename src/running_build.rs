use tokio::sync::broadcast;

type Channel = (broadcast::Sender<String>, broadcast::Receiver<String>);
pub struct RunningBuild {
    pub channel: Channel,
    pub logs: Vec<String>,
}

impl RunningBuild {
    pub fn new(channel: Channel) -> Self {
        Self {
            channel,
            logs: vec![],
        }
    }
}
