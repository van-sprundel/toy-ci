use axum::extract::Path;
use axum::response::sse::Event;
use axum::response::Sse;
use axum::routing::get;
use axum::{routing::post, Router};
use axum::{Extension, Json};
use error::Error;
use events::actor::{Actor, ActorHandler};
use events::new_build_message::NewBuildMessage;
use futures::stream::Stream;
use running_build::RunningBuild;
use std::collections::HashMap;
use std::convert::Infallible;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::{broadcast, mpsc::Receiver, mpsc::Sender, Mutex};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use webhook_payloads::github::GithubPushWebhookPayload;

mod build_executor;
mod error;
mod events;
mod git;
mod running_build;
mod webhook_payloads;

struct AppState {
    builds: Mutex<HashMap<String, RunningBuild>>,
}

impl AppState {
    pub async fn send_log(&self, build_id: &str, message: &str) -> Result<(), Error> {
        let mut build_progress_channel_map = self.builds.lock().await;

        if let Some(build) = build_progress_channel_map.get_mut(build_id) {
            let (tx, _) = &build.channel;
            tx.send(message.to_string())?;

            build.logs.push(message.to_string());
        }

        Ok(())
    }
}

async fn sse_handler(
    Path(build_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    tracing::info!("Receiving sse event");

    let (rx, previous_logs) = {
        let build_progress_tx_map = state.builds.lock().await;
        if let Some(build) = build_progress_tx_map.get(&build_id) {
            let rx = build.channel.0.subscribe();
            let previous_logs = build.logs.clone();
            (Some(rx), previous_logs)
        } else {
            (None, vec![])
        }
    };

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

async fn webhook_handler(
    Extension(build_queue): Extension<ActorHandler>,
    Extension(state): Extension<Arc<AppState>>,
    Json(payload): Json<GithubPushWebhookPayload>,
) -> Json<()> {
    tracing::info!("Received webhook");

    for commit in payload.commits {
        let id = uuid::Uuid::new_v4().to_string();
        let (tx, rx) = broadcast::channel(100);

        state
            .builds
            .lock()
            .await
            .insert(id.clone(), RunningBuild::new((tx, rx)));

        let message = NewBuildMessage {
            commit: commit.into(),
            id: id.clone(),
        };
        let _ = build_queue.send(message).await;
    }

    Json(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, rx): (Sender<NewBuildMessage>, Receiver<NewBuildMessage>) = mpsc::channel(100);
    let build_queue = ActorHandler::new(tx);

    let app_state = Arc::new(AppState {
        builds: Mutex::new(HashMap::default()),
    });

    let app = Router::new()
        .route("/sse/:build_id", get(sse_handler))
        .route("/build", post(webhook_handler))
        .layer(Extension(app_state.clone()))
        .layer(Extension(build_queue));

    let scheduler = tokio::spawn(async move {
        Actor::new(rx).build_scheduler(app_state).await;
    });

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    scheduler.await?;

    Ok(())
}
