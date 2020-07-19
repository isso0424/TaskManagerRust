use crate::args::parse_arg;
use crate::command::types::label::Labels;
use crate::command::types::task::{Task, Tasks};

fn regeneration_task(
    title: String,
    args: Vec<String>,
    mut tasks: Tasks,
    labels: Labels,
) -> Result<Tasks, String> {
    if !tasks.content.iter().any(|task| task.title == title) {
        return Err("Task not found".to_string());
    }

    let task = tasks
        .content
        .iter()
        .find(|task| task.title == title)
        .unwrap();

    let index = tasks.get_index(title)?;

    let new_title = parse_arg::get_title(&args).unwrap_or(task.title.clone());

    let new_limit = parse_arg::get_limit(&args)
        .map(|limit| Some(limit))
        .unwrap_or(task.limit);

    let new_labels = parse_arg::get_label(&args)
        .map(|value| labels.create_label_vec(&value))
        .unwrap_or(task.label.clone());

    tasks.content[index] = Task {
        title: new_title,
        label: new_labels,
        limit: new_limit,
        done: task.done,
    };

    Ok(tasks)
}

fn update_task(title: String, args: Vec<String>) -> Result<(), String> {
    let all_labels = Labels::load().unwrap_or(Labels { content: vec![] });
    let tasks = Tasks::load().map_err(|err| err.to_string())?;
    let tasks = regeneration_task(title, args, tasks, all_labels)?;

    tasks.save().map_err(|err| err.to_string())?;
    Ok(())
}

pub fn update(args: Vec<String>) -> Result<(), String> {
    let title = &args[3];

    update_task(title.clone(), args)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::command::types::label::Label;

    #[test]
    fn regeneration_task_success() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: false,
            }],
        };
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };
        let title = "target".to_string();
        let args = vec![
            "--limit",
            "1999-12-12",
            "--label",
            "label",
            "--title",
            "title",
        ]
        .iter()
        .map(|arg| arg.to_string())
        .collect();

        assert_eq!(
            regeneration_task(title, args, tasks, labels).unwrap(),
            Tasks {
                content: vec![Task {
                    title: "title".to_string(),
                    label: Some(vec![Label {
                        title: "label".to_string()
                    }]),
                    limit: Some(944956800),
                    done: false,
                }],
            }
        );
    }

    #[test]
    fn regeneration_task_failed() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: false,
            }],
        };
        let labels = Labels {
            content: vec![Label {
                title: "label".to_string(),
            }],
        };
        let args = vec![
            "--limit",
            "1999-12-12",
            "--label",
            "label",
            "--title",
            "title",
        ]
        .iter()
        .map(|arg| arg.to_string())
        .collect();

        assert_eq!(
            regeneration_task("".to_string(), args, tasks, labels).ok(),
            None
        );
    }
}
