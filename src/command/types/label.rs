use serde::{Deserialize, Serialize};
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Label {
    pub title: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
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

    pub fn parse(raw_labels: Option<Vec<&str>>, all_labels: Labels) -> Option<Vec<Label>> {
        let labels = &mut vec![];
        if raw_labels.is_none() {
            return None;
        }
        for raw_label in match raw_labels {
            Some(value) => value,
            None => return None,
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

        if labels.is_empty() {
            return None;
        }

        Some(labels.clone())
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

    pub fn create_label_vec(title_vec: &[&str], all_labels: Self) -> Option<Vec<Label>> {
        let mut labels = vec![];

        for title in title_vec {
            if !all_labels.content.iter().any(|label| label.title == *title) {
                return None;
            }

            labels.push(Label {
                title: title.to_string(),
            })
        }

        Some(labels)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_success() {
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(
            Labels::parse(Some(vec!["label"]), labels).unwrap(),
            vec![Label {
                title: "label".to_string()
            }]
        );
    }

    #[test]
    fn parse_failed() {
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(Labels::parse(Some(vec!["invalid"]), labels.clone()), None);
        assert_eq!(Labels::parse(None, labels), None);
    }

    #[test]
    fn search_with_title_success() {
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(
            labels.clone().search_with_title(Some("label".to_string())),
            labels
        );
        assert_eq!(
            labels.search_with_title(Some("s".to_string())),
            Labels { content: vec![] }
        );
    }

    #[test]
    fn create_label_vec_success() {
        let labels = Labels {
            content: vec![
                Label {
                    title: "label1".to_string(),
                },
                Label {
                    title: "label2".to_string(),
                },
                Label {
                    title: "label3".to_string(),
                },
            ],
        };

        assert_eq!(
            Labels::create_label_vec(&vec!["label1", "label3"], labels).unwrap(),
            vec![
                Label {
                    title: "label1".to_string()
                },
                Label {
                    title: "label3".to_string()
                }
            ]
        );
    }

    #[test]
    fn create_label_vec_failed() {
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(Labels::create_label_vec(&vec!["invalid"], labels), None);
    }
}
