use thiserror::Error;

pub type Result<T> = anyhow::Result<T>;

#[derive(Error, Debug)]
pub enum MerelError {
    #[error("Could not execute command {0}, got {1}")]
    CommandFailed(String, String),
}
