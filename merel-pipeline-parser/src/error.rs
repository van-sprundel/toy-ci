use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum PipelineParserError {
    #[error("Container method {0} is not supported.")]
    UnavailableContainerMethod(String),
}

