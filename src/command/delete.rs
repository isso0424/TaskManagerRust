use crate::command::types::{label::Labels, task::Tasks};

fn delete_task(title: String) -> Result<(), String> {
    let task_not_found = "Task not found".to_string();
    let mut tasks = Tasks::load().map_err(|err| err.to_string())?;

    if tasks.content.iter().any(|task| task.title == title) {
        let new_tasks = tasks
            .content
            .drain(..)
            .filter(|task| task.title != title)
            .collect();

        tasks.content = new_tasks;
        tasks.save().map_err(|err| err.to_string())?;

        return Ok(());
    }

    Err(task_not_found)
}

fn delete_label(title: String) -> Result<(), String> {
    let label_not_found = "Label not found".to_string();
    let mut labels = Labels::load().map_err(|err| err.to_string())?;

    if labels.content.iter().any(|label| label.title == title) {
        let new_labels = labels
            .content
            .drain(..)
            .filter(|label| label.title != title)
            .collect();
        labels.content = new_labels;
        labels.save().map_err(|err| err.to_string())?;

        return Ok(());
    }

    Err(label_not_found)
}

pub fn delete(args: Vec<String>) -> Result<(), String> {
    //
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
