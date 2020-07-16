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
    let tasks = match Tasks::load() {
        Ok(tasks) => update_task(tasks, title)?,
        Err(err) => return Err(err.to_string()),
    };

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
