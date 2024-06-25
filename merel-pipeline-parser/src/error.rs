use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum PipelineParserError {
}

