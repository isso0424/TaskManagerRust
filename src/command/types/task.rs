use serde::{Deserialize, Serialize};

use crate::command::types::label::Label;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Task {
    pub title: String,
    pub label: Option<Vec<Label>>,
    pub limit: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Tasks {
    pub content: Vec<Task>,
}
