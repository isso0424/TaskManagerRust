use serde::{Deserialize, Serialize};

use crate::command::types::label::Label;

#[derive(Serialize, Deserialize)]
pub struct Task {
    title: String,
    label: Vec<Label>,
    limit: u64,
}
