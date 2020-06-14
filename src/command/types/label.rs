use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Label {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Labels {
    pub content: Vec<Label>,
}
