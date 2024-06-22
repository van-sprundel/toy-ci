use crate::build_context::BuildContext;

#[derive(Clone)]
pub struct NewBuildMessage {
    pub context: BuildContext,
}
