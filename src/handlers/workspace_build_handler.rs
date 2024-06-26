use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use uuid::Uuid;

use crate::app_state::AppState;
use crate::command::run_command;
use crate::webhook_payloads::github::GithubPushWebhookPayload;
use crate::{MerelError, Result};
use crate::{
    events::{
        actor::ActorHandler,
        build_message::{BuildMessage, NewBuildMessage},
    },
    git::commit::Commit,
    pipeline::Pipeline,
    workspace_context::WorkspaceContext,
};

pub async fn workspace_build_handler(
    Path(_workspace_id): Path<String>,
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
    let workspace_id = Uuid::new_v4().to_string();
    tracing::info!("Workspace {workspace_id} created");

    let repository_name = payload.repository.name.clone();
    let url = payload.repository.url.clone();
    let repo_dir = format!("/tmp/merel/repositories/{}-{}", repository_name, commit.id);

    let context = WorkspaceContext {
        id: workspace_id.clone(),
        commit_id: commit.id,
        repo_url: url,
        repo_dir,
    };

    let output = state.create_git_directory_if_not_exists(&context).await;
    if let Err(e) = output {
        state.send_log(&workspace_id, &e.to_string()).await;

        let span = tracing::error_span!("Can't create git repository");
        span.in_scope(|| {
            tracing::error!("{e}");
        });

        return;
    }

    // checkout git
    let checkout = run_command(
        state,
        &workspace_id.to_string(),
        "git",
        Some(vec!["checkout", &context.commit_id]),
        Some(&context.repo_dir),
    )
    .await;
    if let Err(e) = checkout {
        state.send_log(&workspace_id, &e.to_string()).await;

        let span = tracing::error_span!("Can't checkout repository");
        span.in_scope(|| {
            tracing::error!("{e}");
        });

        return;
    }

    // get pipeline in curr workspace
    let pipeline = match get_pipeline(&context).await {
        Ok(v) => v,
        Err(e) => {
            state.send_log(&workspace_id, &e.to_string()).await;

            let span = tracing::error_span!("Can't retrieve pipeline");
            span.in_scope(|| {
                tracing::error!("{e}");
            });

            return;
        }
    };

    // TODO: replace with actual branch
    if !pipeline.should_trigger("main") {
        tracing::debug!("No trigger needed.");
        return;
    }

    state.create_workspace(&context.id).await;

    let message = NewBuildMessage { context, pipeline };
    let _ = build_queue.send(BuildMessage::NewBuild(message)).await;
}

async fn get_pipeline(context: &WorkspaceContext) -> Result<Pipeline> {
    let repo_dir = &context.repo_dir;
    let path = std::path::Path::new(&repo_dir.clone()).join("merel.yaml");

    if !path.exists() {
        return Err(MerelError::PipelineRetrieveError(repo_dir.to_string()).into());
    }

    let content = match std::fs::read_to_string(&path) {
        Ok(v) => v,
        Err(_) => {
            return Err(MerelError::PipelineRetrieveError(repo_dir.to_string()).into());
        }
    };

    // parse pipeline
    serde_yaml::from_str::<Pipeline>(&content).map_err(|e| e.into())
}
