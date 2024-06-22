mod app_state;
mod build_context;
mod build_executor;
mod events;
mod git;
mod running_build;
mod webhook_payloads;

use app_state::AppState;
use axum::extract::Path;
use axum::response::sse::Event;
use axum::response::Sse;
use axum::routing::get;
use axum::{routing::post, Router};
use axum::{Extension, Json};
use build_context::BuildContext;
use events::actor::{Actor, ActorHandler};
use events::new_build_message::NewBuildMessage;
use futures::stream::Stream;
use git::commit::Commit;
use merel::Result;
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::process::Command;
use tokio::sync::mpsc;
use tokio::sync::{mpsc::Receiver, mpsc::Sender};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use uuid::Uuid;
use webhook_payloads::github::GithubPushWebhookPayload;

async fn sse_handler(
    Path(build_id): Path<String>,
    Extension(state): Extension<Arc<AppState>>,
) -> Sse<impl Stream<Item = Result<Event>>> {
    tracing::info!("Receiving sse event");

    let (rx, previous_logs) = {
        let builds = state.get_builds();
        if let Some(build) = builds.lock().await.get(&build_id) {
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

    for commit in &payload.commits {
        let commit: Commit = commit.clone().into();
        process_commit(&build_queue, &state, &payload, commit).await;
    }

    Json(())
}

async fn process_commit(
    build_queue: &ActorHandler,
    state: &Arc<AppState>,
    payload: &GithubPushWebhookPayload,
    commit: Commit,
) {
    let build_id = Uuid::new_v4();
    println!("Build context {build_id} created");

    let repository_name = payload.repository.name.clone();
    let url = payload.repository.url.clone();
    let repo_dir = format!("/tmp/merel/{}-{}", repository_name, commit.id);

    let context = BuildContext {
        id: build_id.to_string(),
        commit_id: commit.id,
        repo_url: url,
        repo_dir,
    };

    let output = state.create_git_directory_if_not_exists(&context).await;
    if let Err(e) = output {
        let span = tracing::error_span!("Can't create git repository");
        span.in_scope(|| {
            tracing::error!("{e}");
        });

        return;
    }

    let checkout = Command::new("git")
        .arg0("checkout")
        .current_dir(&context.repo_dir)
        .output()
        .await;
    if let Err(e) = checkout {
        let span = tracing::error_span!("Can't checkout repository");
        span.in_scope(|| {
            tracing::error!("{e}");
        });

        return;
    }
    return;

    // get pipeline in curr build

    // parse pipeline

    // if it can't be parsed then return build state CantParse
    // if it can parse, check if it should trigger the current branch,

    // if so then start build

    state.create_build(&context.id).await;

    let message = NewBuildMessage { context };
    let _ = build_queue.send(message).await;
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, rx): (Sender<NewBuildMessage>, Receiver<NewBuildMessage>) = mpsc::channel(100);
    let build_queue = ActorHandler::new(tx);

    let app_state = Arc::new(AppState::default());

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
