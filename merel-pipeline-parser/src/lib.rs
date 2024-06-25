mod error;

use error::Result;
use merel_core::pipeline::Pipeline;

pub struct PipelineParser {
    input: String,
}

impl PipelineParser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string()
        }
    }

    pub fn parse(&self) -> Result<Pipeline> {
        let mut jobs = vec![];


        Ok(Pipeline {
            jobs,
            trigger_branch: "".to_string()
        })
    }
}
