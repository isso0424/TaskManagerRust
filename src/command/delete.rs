use crate::command::types::{label::Labels, task::Tasks};

fn update_task(mut tasks: Tasks, title: String) -> Result<Tasks, String> {
    if !tasks.content.iter().any(|task| task.title == title) {
        return Err("Task not found".to_string());
    }

    Ok(Tasks {
        content: tasks
            .content
            .drain(..)
            .filter(|task| task.title != title)
            .collect(),
    })
}

fn delete_task(title: String) -> Result<(), String> {
    let tasks = Tasks::load().map_err(|err| err.to_string())?;
    let tasks = update_task(tasks, title)?;

    tasks.save().map_err(|err| err.to_string())?;

    Ok(())
}

fn update_label(mut labels: Labels, title: String) -> Result<Labels, String> {
    if !labels.content.iter().any(|label| label.title == title) {
        return Err("Label not found".to_string());
    }

    Ok(Labels {
        content: labels
            .content
            .drain(..)
            .filter(|label| label.title != title)
            .collect(),
    })
}

fn delete_label(title: String) -> Result<(), String> {
    let labels = match Labels::load() {
        Ok(labels) => update_label(labels, title)?,
        Err(err) => return Err(err.to_string()),
    };

    labels.save().map_err(|err| err.to_string())?;

    Ok(())
}

pub fn delete(args: Vec<String>) -> Result<(), String> {
    let target = &args[2];
    let title = &args[3];

    match target.as_str() {
        "task" => delete_task(title.clone())?,
        "label" => delete_label(title.clone())?,
        _ => {
            return Err("target not found".to_string());
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::types::{label::Label, task::Task};
    #[test]
    fn update_task_success() {
        let tasks = Tasks {
            content: vec![
                Task {
                    title: "title".to_string(),
                    label: None,
                    limit: None,
                    done: false,
                },
                Task {
                    title: "invalid".to_string(),
                    label: None,
                    limit: None,
                    done: false,
                },
            ],
        };

        assert_eq!(
            update_task(tasks.clone(), "title".to_string()).unwrap(),
            Tasks {
                content: vec![Task {
                    title: "invalid".to_string(),
                    label: None,
                    limit: None,
                    done: false,
                }]
            }
        );
    }

    #[test]
    fn update_task_failed() {
        let tasks = Tasks {
            content: vec![
                Task {
                    title: "title".to_string(),
                    label: None,
                    limit: None,
                    done: false,
                },
                Task {
                    title: "invalid".to_string(),
                    label: None,
                    limit: None,
                    done: false,
                },
            ],
        };

        assert_eq!(update_task(tasks, "".to_string()).ok(), None);

        let tasks = Tasks { content: vec![] };

        assert_eq!(update_task(tasks, "".to_string()).ok(), None);
    }

    #[test]
    fn update_label_success() {
        let labels = Labels {
            content: vec![Label {
                title: "title".to_string(),
            }],
        };

        assert_eq!(
            update_label(labels, "title".to_string()).unwrap(),
            Labels { content: vec![] }
        );
    }

    #[test]
    fn update_label_failed() {
        let labels = Labels {
            content: vec![Label {
                title: "title".to_string(),
            }],
        };

        assert_eq!(update_label(labels, "".to_string()).ok(), None);

        let labels = Labels { content: vec![] };
        assert_eq!(update_label(labels, "".to_string()).ok(), None);
    }
}
