use std::{iter::Peekable, str::Chars};

use crate::error::{PipelineParserError, Result};
use merel_core::{container_method::ContainerMethod, pipeline::Pipeline};

pub struct PipelineParser {
    input: String,
}

impl PipelineParser {
    pub fn new(input: &str) -> Self {
        Self {
            input: input.to_string(),
        }
    }

    pub fn parse(&self) -> Result<Pipeline> {
        let jobs = vec![];

        let mut peekable = self.input.chars().peekable();

        let container_method = match self.parse_container_method(&mut peekable) {
            Ok(v) => v,
            Err(e) => {
                return Err(e);
            }
        };

        Ok(Pipeline {
            jobs,
            container_method,
            trigger_branches: vec!["main".to_string()],
        })
    }

    fn eat(peekable: &mut Peekable<Chars>) {
        while let Some(current) = peekable.peek() {
            if current.is_whitespace() || current == &'\r' || current == &'\n' {
                peekable.next();
            } else {
                break;
            }
        }
    }

    fn parse_container_method(&self, peekable: &mut Peekable<Chars>) -> Result<ContainerMethod> {
        let mut container_method = String::new();

        'main: while let Some(current) = peekable.peek() {
            match current {
                '{' => {
                    peekable.next();
                    Self::eat(peekable);
                    while let Some(current) = peekable.peek() {
                        // parse_jobs
                        if *current == '}' {
                            break 'main;
                        }
                    }
                }
                _ => {
                    if current.is_alphanumeric() {
                        let current = peekable.next().unwrap();
                        container_method.push(current);
                    } else if current.is_whitespace() {
                        peekable.next();
                    }
                }
            }
        }

        match &*container_method {
            "docker" => Ok(ContainerMethod::Docker),
            _ => Err(PipelineParserError::UnavailableContainerMethod(container_method).into()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_method_should_succeed() {
        let possible_configs = [
            "docker{}",
            "docker  {}",
            "docker{  }",
            "docker    {}",
            "docker   {     }",
        ];

        for possible_config in possible_configs {
            let mut pipeline_parser = PipelineParser::new(possible_config);

            let pipeline = pipeline_parser.parse().unwrap();
            assert_eq!(
                pipeline,
                Pipeline {
                    container_method: ContainerMethod::Docker,
                    jobs: vec![],
                    trigger_branches: "".to_string()
                }
            )
        }
    }
}
