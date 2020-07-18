use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

use chrono::{Local, TimeZone};

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
        let keyword = match title {
            Some(v) => v,
            None => return self,
        };

        let searched_tasks = self
            .content
            .drain(..)
            .filter(|task| task.title.contains(keyword.as_str()))
            .collect();

        Tasks {
            content: searched_tasks,
        }
    }

    pub fn search_with_label(mut self, target_label: Option<String>) -> Self {
        let keyword = match target_label {
            Some(v) => v,
            None => return self,
        };

        let searched_tasks = self
            .content
            .drain(..)
            .filter(|task| match &task.label {
                Some(task) => task.iter().any(|label| label.title == keyword),
                None => false,
            })
            .collect();

        Tasks {
            content: searched_tasks,
        }
    }
}

impl Task {
    pub fn get_title(&self) -> String {
        self.title.clone()
    }

    pub fn get_limit(&self) -> i64 {
        match self.limit {
            Some(limit) => limit,
            None => 0,
        }
    }

    pub fn get_label(&self) -> Vec<Label> {
        match &self.label {
            Some(labels) => labels.clone(),
            None => vec![],
        }
    }

    pub fn limit_to_string(&self) -> String {
        match self.get_limit() {
            0 => "なし".to_string(),
            limit => Local.timestamp(limit, 0).to_string(),
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
    fn get_title_test() {
        let task = Task {
            title: "target".to_string(),
            label: None,
            limit: None,
            done: false,
        };

        assert_eq!(task.get_title(), task.title);
    }

    #[test]
    fn get_limit_success() {
        let mut task = Task {
            title: "target".to_string(),
            label: None,
            limit: Some(110000),
            done: false,
        };

        assert_eq!(task.get_limit(), 110000);

        task.limit = None;
        assert_eq!(task.get_limit(), 0);
    }

    #[test]
    fn get_label_success() {
        let mut task = Task {
            title: "target".to_string(),
            label: Some(vec![Label {
                title: "label".to_string(),
            }]),
            limit: None,
            done: false,
        };

        assert_eq!(
            task.get_label(),
            vec![Label {
                title: "label".to_string()
            }]
        );

        task.label = None;
        assert_eq!(task.get_label(), vec![]);
    }
}
