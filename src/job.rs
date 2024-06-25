use crate::error::Result;
use crate::step::Step;

#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct Job {
    image: String,
    steps: Vec<Step>,
}

impl Job {
    pub fn run(&self) -> Result<()> {
        for _step in &self.steps {
            // run step
        }

        Ok(())
    }
}
