use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Label {
    title: String,
}
