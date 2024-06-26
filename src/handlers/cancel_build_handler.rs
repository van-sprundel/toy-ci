use axum::{extract::Path, Extension, Json};
use serde_json::json;

use crate::events::{
    actor::ActorHandler,
    build_message::{BuildMessage, CancelBuildMessage},
};

pub async fn cancel_build_handler(
    Path(workspace_id): Path<String>,
    Extension(build_queue): Extension<ActorHandler>,
) -> Json<serde_json::Value> {
    tracing::info!("Cancelling build with id: {}", workspace_id);

    let message = CancelBuildMessage(workspace_id.clone());
    let _ = build_queue.send(BuildMessage::CancelBuild(message)).await;

    Json(json!({ "status": "success", "message": "Build cancelled successfully" }))
}
