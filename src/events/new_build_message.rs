use crate::pipeline::Pipeline;

use crate::workspace_context::WorkspaceContext;

#[derive(Clone)]
pub struct NewBuildMessage {
    pub context: WorkspaceContext,
    pub pipeline: Pipeline,
}
