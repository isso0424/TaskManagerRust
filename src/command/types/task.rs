use serde::{Deserialize, Serialize};

use crate::command::types::label::Label;

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    label: Option<Vec<Label>>,
    limit: Option<u64>,
}

#[derive(Serialize, Deserialize)]
pub struct Tasks {
    pub content: Vec<Task>,
}
