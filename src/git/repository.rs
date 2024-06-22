use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone)]
pub struct Repository {
    pub name: String,
    pub clone_url: String,
}
