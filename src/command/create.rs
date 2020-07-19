use crate::command::types::{
    label::{Label, Labels},
    task::{Task, Tasks},
};

use crate::args::parse_arg::{get_label, get_limit};

fn update_label(mut labels: Labels, title: &str) -> Result<Labels, String> {
    if labels.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate label title".to_string());
    }

    let new_label = Label {
        title: title.to_string(),
    };

    labels.content.push(new_label);

    Ok(labels)
}

fn create_label(title: &str) -> Result<(), String> {
    let labels = match Labels::load() {
        Ok(labels) => update_label(labels, title)?,
        Err(err) => return Err(err.to_string()),
    };
    labels.save().map_err(|err| err.to_string())?;

    Ok(())
}

fn update_task<'a>(
    tasks: &'a mut Tasks,
    title: &str,
    label: Option<Vec<&str>>,
    limit: Option<i64>,
    all_labels: Labels,
) -> Result<&'a Tasks, String> {
    if tasks.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate task title".to_string());
    }

    if title == "" {
        return Err("Cannot use empty task title".to_string());
    }

    let new_task = Task {
        title: title.to_string(),
        label: Labels::parse(label, all_labels),
        limit,
        done: false,
    };

    tasks.content.push(new_task);

    Ok(tasks)
}

fn create_task(title: &str, label: Option<Vec<&str>>, limit: Option<i64>) -> Result<(), String> {
    let all_labels = Labels::load().map_err(|err| err.to_string())?;
    let tasks = &mut Tasks::load().map_err(|err| err.to_string())?;
    update_task(tasks, title, label, limit, all_labels)?;
    tasks.save().map_err(|err| err.to_string())?;

    Ok(())
}

pub fn create(args: Vec<String>) -> Result<(), String> {
    let invalid_target = "Invalid Target".to_owned();

    let target = &args[2];
    let title = &args[3];

    match target.as_str() {
        "label" => {
            create_label(title.as_str())?;
        }
        "task" => {
            let label = get_label(&args);
            let limit = get_limit(&args);

            create_task(title, label, limit)?;
        }

        invalid => {
            error!("{} is invalid target", invalid);
            return Err(invalid_target);
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_label_success() {
        let labels = Labels { content: vec![] };
        let title = "title";
        assert_eq!(
            update_label(labels, title).unwrap(),
            Labels {
                content: vec![Label {
                    title: title.to_string()
                }]
            }
        );
    }

    #[test]
    fn update_label_failed() {
        let title = "title";
        let labels = Labels {
            content: vec![Label {
                title: title.to_string(),
            }],
        };
        assert_eq!(update_label(labels, title).ok(), None);
    }

    #[test]
    fn update_task_success() {
        let title = "title";
        let limit = Some(114514);
        let label = Some(vec!["label"]);
        let tasks = &mut Tasks { content: vec![] };
        let all_labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(
            *update_task(tasks, title, label, limit, all_labels.clone()).unwrap(),
            Tasks {
                content: vec![Task {
                    title: title.to_string(),
                    label: Some(vec![Label {
                        title: "label".to_string()
                    }]),
                    limit,
                    done: false
                }]
            }
        );

        let second_title = "second";

        assert_eq!(
            *update_task(tasks, second_title, None, None, all_labels).unwrap(),
            Tasks {
                content: vec![
                    Task {
                        title: title.to_string(),
                        label: Some(vec![Label {
                            title: "label".to_string()
                        }]),
                        limit,
                        done: false
                    },
                    Task {
                        title: second_title.to_string(),
                        label: None,
                        limit: None,
                        done: false
                    }
                ]
            }
        );
    }

    #[test]
    fn update_task_failed() {
        let title = "title";
        let limit = None;
        let label = Some(vec!["label"]);
        let tasks = &mut Tasks {
            content: vec![Task {
                title: "title".to_string(),
                label: Some(vec![Label {
                    title: "label".to_string(),
                }]),
                limit,
                done: false,
            }],
        };
        let all_labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };

        assert_eq!(
            update_task(tasks, title, label.clone(), limit, all_labels.clone()).ok(),
            None
        );

        assert_eq!(update_task(tasks, "", label, limit, all_labels).ok(), None)
    }
}
