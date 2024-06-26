use crate::pipeline::Pipeline;

use crate::workspace_context::WorkspaceContext;
pub enum BuildMessage {
    NewBuild(NewBuildMessage),
    CancelBuild(CancelBuildMessage),
}

#[derive(Clone)]
pub struct NewBuildMessage {
    pub context: WorkspaceContext,
    pub pipeline: Pipeline,
}

pub struct CancelBuildMessage(pub String);
