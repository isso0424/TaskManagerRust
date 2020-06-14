use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct Labels {
    pub content: Vec<Label>,
}
