use crate::command::types::label::{Label, Labels};
use crate::command::types::task::{Task, Tasks};
use crate::config::parse_arg;

fn update_task(title: String, args: Vec<String>) -> Result<(), String> {
    let mut tasks = Tasks::load().map_err(|err| err.to_string())?;

    if !tasks.content.iter().any(|task| task.title == title) {
        return Err("Task not found".to_string());
    }

    let task = tasks
        .content
        .iter()
        .find(|task| task.title == title)
        .unwrap();

    let index = Tasks::get_index(title)?;
    let new_title = match parse_arg::get_title(&args) {
        Some(value) => value,
        None => task.get_title(),
    };
    let new_limit = match parse_arg::get_limit(&args) {
        Some(value) => Some(value),
        None => task.limit,
    };
    let new_labels = match parse_arg::get_label(&args) {
        Some(value) => Labels::create_label_vec(value),
        None => task.label,
    };

    tasks.content[index] = Task {
        title: new_title,
        label: new_labels,
        limit: new_limit,
        done: task.done,
    };

    tasks.save().map_err(|err| err.to_string())?;
    Ok(())
}

pub fn update(args: Vec<String>) -> Result<(), String> {
    let title = &args[3];

    update_task(title.clone(), args)?;

    Ok(())
}
