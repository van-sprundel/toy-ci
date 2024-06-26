use std::sync::Arc;

use axum::response::Sse;
use axum::Extension;
use axum::{extract::Path, response::sse::Event};
use futures::Stream;

use crate::app_state::AppState;
use crate::prelude::LOGS_DIR;
use crate::Result;

pub async fn build_sse_handler(
    Path(build_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event>>> {
    tracing::info!("Receiving sse event");

    let (rx, mut previous_logs) = {
        let builds = state.get_builds();
        if let Some(build) = builds.lock().await.get(&build_id) {
            let rx = build.channel.0.subscribe();
            let previous_logs = build.logs.clone();
            tracing::trace!("Adding logs from active build");

            (Some(rx), previous_logs)
        } else {
            (None, vec![])
        }
    };

    // check for log file and add contents to previous_logs if it exists
    let log_path = format!("{}/{}-logs.txt", LOGS_DIR, build_id);
    tracing::trace!("Checking logs file {log_path}");
    if std::path::Path::new(&log_path).exists() {
        match std::fs::read_to_string(&log_path) {
            Ok(contents) => {
                tracing::trace!("Adding logs from log file");
                previous_logs.push(contents);
            }
            Err(e) => {
                tracing::error!("Failed to read log file: {}", e);
            }
        }
    }

    let stream = async_stream::stream! {
        for log in previous_logs {
            yield Ok(Event::default().data(log));
        }

        if let Some(mut rx) = rx {
            while let Ok(message) = rx.recv().await {
                yield Ok(Event::default().data(message));
           }
        } else {
            yield Ok(Event::default().data("No events found for this repository"));
        }
    };

    Sse::new(stream)
}
