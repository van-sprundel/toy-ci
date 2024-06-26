use crate::pipeline::Pipeline;

use crate::workspace_context::BuildContext;
pub enum BuildMessage {
    NewBuild(NewBuildMessage),
    CancelBuild(CancelBuildMessage),
}

#[derive(Clone)]
pub struct NewBuildMessage {
    pub context: BuildContext,
    pub pipeline: Pipeline,
}

pub struct CancelBuildMessage(pub String);
