use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Label {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Labels {
    pub content: Vec<Label>,
}
