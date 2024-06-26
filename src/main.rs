mod app_state;
mod build_executor;
mod command;
mod error;
mod events;
mod git;
mod handlers;
mod job;
mod pipeline;
mod prelude;
mod running_build;
mod step;
mod webhook_payloads;
mod workspace_context;

use std::sync::Arc;

use app_state::AppState;
use axum::http::{HeaderValue, Method};
use axum::Extension;
use axum::{
    routing::{get, post, put},
    Router,
};
pub use error::*;
use events::actor::{Actor, ActorHandler};
use events::build_message::BuildMessage;
use handlers::*;
use tokio::net::TcpListener;
use tokio::sync::mpsc;
use tokio::sync::{mpsc::Receiver, mpsc::Sender};
use tower_http::cors::CorsLayer;
use tower_http::services::ServeDir;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let (tx, rx): (Sender<BuildMessage>, Receiver<BuildMessage>) = mpsc::channel(100);
    let build_queue = ActorHandler::new(tx);

    let app_state = Arc::new(AppState::default());
    let static_files = ServeDir::new("dist");

    let app = Router::new()
        .route("/builds/:build_id/sse", get(build_sse_handler))
        .route("/builds/:build_id/cancel", put(cancel_build_handler))
        .route(
            "/workspaces/:workspace_id/build",
            post(workspace_build_handler),
        )
        .layer(
            CorsLayer::new()
                .allow_origin("http://127.0.0.1:5173".parse::<HeaderValue>().unwrap())
                .allow_methods([Method::GET]),
        )
        .layer(Extension(app_state.clone()))
        .layer(Extension(build_queue))
        .fallback_service(static_files.clone());

    let scheduler = tokio::spawn(async move {
        Actor::new(rx).build_scheduler(app_state).await;
    });

    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    scheduler.await?;

    Ok(())
}
