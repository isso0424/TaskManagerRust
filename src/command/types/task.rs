use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

use chrono::{TimeZone, Utc};

use crate::command::types::label::Label;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Task {
    pub title: String,
    pub label: Option<Vec<Label>>,
    pub limit: Option<i64>,
    pub done: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

    pub fn get_index(title: String, tasks: &Tasks) -> Result<usize, String> {
        for (i, task) in tasks.content.iter().enumerate() {
            if task.title == title {
                return Ok(i);
            }
        }

        Err("task not found".to_string())
    }

    pub fn search_with_title(mut self, title: Option<String>) -> Self {
        if let Some(keyword) = title {
            let searched_tasks = self
                .content
                .drain(..)
                .filter(|task| task.title.contains(keyword.as_str()))
                .collect();

            return Tasks {
                content: searched_tasks,
            };
        }

        self
    }

    pub fn search_with_label(mut self, target_label: Option<String>) -> Self {
        if let Some(keyword) = target_label {
            let searched_tasks = self
                .content
                .drain(..)
                .filter(|task| match &task.label {
                    Some(task) => task.iter().any(|label| label.title == keyword),
                    None => false,
                })
                .collect();

            return Tasks {
                content: searched_tasks,
            };
        }

        self
    }
}

impl Task {
    pub fn limit_to_string(&self) -> String {
        match self.limit {
            None => "なし".to_string(),
            Some(limit) => Utc.timestamp(limit, 0).to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn get_index_success() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: false,
            }],
        };

        assert_eq!(Tasks::get_index("target".to_string(), &tasks).unwrap(), 0);
    }

    #[test]
    fn get_index_failed() {
        let tasks = Tasks { content: vec![] };

        assert_eq!(Tasks::get_index("invalid".to_string(), &tasks).ok(), None);
    }

    #[test]
    fn search_with_title_success() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: false,
            }],
        };

        assert_eq!(
            tasks.clone().search_with_title(Some("target".to_string())),
            tasks.clone()
        );

        assert_eq!(tasks.clone().search_with_title(None), tasks.clone());
        assert_eq!(
            tasks.clone().search_with_title(Some("invalid".to_string())),
            Tasks { content: vec![] }
        );
    }

    #[test]
    fn search_with_label_success() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: Some(vec![Label {
                    title: "label".to_string(),
                }]),
                limit: None,
                done: false,
            }],
        };

        assert_eq!(
            tasks.clone().search_with_label(Some("label".to_string())),
            tasks.clone()
        );

        assert_eq!(
            tasks.clone().search_with_label(Some("invalid".to_string())),
            Tasks { content: vec![] }
        );

        assert_eq!(tasks.clone().search_with_label(None), tasks.clone());
    }

    #[test]
    fn limit_to_string_success() {
        //
        let mut task = Task {
            title: "target".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: Some(944956800),
            done: false,
        };

        assert_eq!(
            task.limit_to_string(),
            "1999-12-12 00:00:00 UTC".to_string()
        );

        task.limit = None;
        assert_eq!(task.limit_to_string(), "なし".to_string());
    }
}
