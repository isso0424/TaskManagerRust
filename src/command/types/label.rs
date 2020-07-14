use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Label {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Labels {
    pub content: Vec<Label>,
}

impl Labels {
    pub fn load() -> Result<Labels, std::io::Error> {
        let dir_path = current_exe()?.parent().unwrap().join("labels.json");

        let file = File::open(dir_path)?;

        let label_json = serde_json::from_reader(file)?;

        Ok(label_json)
    }

    pub fn parse(raw_labels: Option<Vec<&str>>) -> Result<Option<Vec<Label>>, String> {
        //
        let labels = &mut vec![];
        if raw_labels.is_none() {
            return Ok(None);
        }
        let all_labels = Labels::load().map_err(|err| err.to_string())?;
        for raw_label in match raw_labels {
            Some(value) => value,
            None => return Ok(None),
        } {
            if !all_labels
                .content
                .iter()
                .any(|label| label.title == raw_label)
            {
                continue;
            }
            labels.push(Label {
                title: raw_label.to_string(),
            });
        }

        Ok(Some(labels.clone()))
    }

    pub fn save(&self) -> Result<(), std::io::Error> {
        let dir_path = current_exe()?.parent().unwrap().join("labels.json");

        let json = serde_json::to_string(self)?;

        let file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(dir_path)?;

        let mut writer = BufWriter::new(file);

        write!(writer, "{}", json)?;

        Ok(())
    }

    pub fn search_with_title(mut self, title: Option<String>) -> Self {
        let keyword = match title {
            Some(v) => v,
            None => return self,
        };

        let searched_labels = self
            .content
            .drain(..)
            .filter(|label| label.title.contains(keyword.as_str()))
            .collect();

        Labels {
            content: searched_labels,
        }
    }

    pub fn create_label_vec(title_vec: &[&str]) -> Option<Vec<Label>> {
        let all_labels = match Self::load() {
            Ok(labels) => labels.content,
            Err(_) => return None,
        };

        let mut labels = vec![];

        for title in title_vec {
            if !all_labels.iter().any(|label| label.title == *title) {
                return None;
            }

            labels.push(Label {
                title: title.to_string(),
            })
        }

        Some(labels)
    }
}
