use crate::command::types::label::Labels;
use crate::command::types::task::{Task, Tasks};
use crate::config::parse_arg;

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

    let index = Tasks::get_index(title, &tasks)?;
    let new_title = match parse_arg::get_title(&args) {
        Some(value) => value,
        None => task.get_title(),
    };
    let new_limit = match parse_arg::get_limit(&args) {
        Some(value) => Some(value),
        None => task.limit,
    };
    let new_labels = match &parse_arg::get_label(&args) {
        Some(value) => Labels::create_label_vec(value, labels),
        None => task.label.clone(),
    };

    tasks.content[index] = Task {
        title: new_title,
        label: new_labels,
        limit: new_limit,
        done: task.done,
    };

    Ok(tasks)
}

fn update_task(title: String, args: Vec<String>) -> Result<(), String> {
    let all_labels = match Labels::load() {
        Ok(labels) => labels,
        Err(_) => Labels { content: vec![] },
    };
    let tasks = match Tasks::load() {
        Ok(tasks) => regeneration_task(title, args, tasks, all_labels)?,
        Err(err) => return Err(err.to_string()),
    };

    tasks.save().map_err(|err| err.to_string())?;
    Ok(())
}

pub fn update(args: Vec<String>) -> Result<(), String> {
    let title = &args[3];

    update_task(title.clone(), args)?;

    Ok(())
}
