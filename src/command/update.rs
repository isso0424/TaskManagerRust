use crate::command::types::label::Label;
use crate::command::types::task::{Task, Tasks};
use crate::config::parse_arg;

fn update_task(title: String, args: Vec<String>) -> Result<(), String> {
    let task_not_found = "Task not found".to_string();
    let mut tasks = Tasks::load().map_err(|err| err.to_string())?;

    if tasks.content.iter().any(|task| task.title == title) {
        let task = tasks
            .content
            .iter()
            .find(|task| task.title == title)
            .unwrap();
        let index = Tasks::get_index(title)?;
        let new_title = match parse_arg::get_title(&args) {
            Some(value) => value,
            None => task.title.clone(),
        };
        let new_limit = match parse_arg::get_limit(&args) {
            Some(value) => Some(value),
            None => task.limit,
        };
        let new_labels = match parse_arg::get_label(&args) {
            Some(value) => Some(value),
            None => match &task.label {
                Some(value) => {
                    let labels = value;
                    Some(labels.iter().map(|label| label.title.as_str()).collect())
                }
                None => None,
            },
        };

        tasks.content[index] = Task {
            title: new_title,
            label: match new_labels {
                Some(labels) => Some(
                    labels
                        .iter()
                        .map(|value| Label {
                            title: value.to_string(),
                        })
                        .collect(),
                ),
                None => None,
            },
            limit: new_limit,
            done: task.done,
        };

        tasks.save().map_err(|err| err.to_string())?;
        return Ok(());
    }

    Err(task_not_found)
}

pub fn update(args: Vec<String>) -> Result<(), String> {
    let title = &args[3];

    update_task(title.clone(), args)?;

    Ok(())
}
