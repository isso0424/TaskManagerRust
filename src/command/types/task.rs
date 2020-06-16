use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

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

impl Tasks {
    pub fn load() -> Result<Tasks, std::io::Error> {
        let dir_path = current_exe()?.parent().unwrap().join("tasks.json");

        let file = File::open(dir_path)?;

        let tasks_json = serde_json::from_reader(file)?;

        Ok(tasks_json)
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let dir_path = current_exe()?.parent().unwrap().join("tasks.json");

        let json = serde_json::to_string(&self)?;

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(dir_path)?;

        let mut writer = BufWriter::new(file);

        write!(writer, "{}", json)?;

        Ok(())
    }
}
