use crate::command::types::task::{Task, Tasks};

fn update_task(mut tasks: Tasks, title: String) -> Result<Tasks, String> {
    if !tasks.content.iter().any(|task| task.title == title) {
        return Err("Task not found".to_string());
    }

    let task = tasks
        .content
        .iter()
        .find(|task| task.title == title)
        .unwrap();
    let index = Tasks::get_index(title)?;

    tasks.content[index] = Task {
        title: task.title.clone(),
        label: task.label.clone(),
        limit: task.limit,
        done: true,
    };

    Ok(tasks)
}

fn mark_done_task(task_title: String) -> Result<(), String> {
    let tasks = match Tasks::load() {
        Ok(tasks) => update_task(tasks, task_title)?,
        Err(err) => return Err(err.to_string()),
    };

    tasks.save().map_err(|err| err.to_string())?;

    Ok(())
}

pub fn done(args: Vec<String>) -> Result<(), String> {
    let target_title = &args[2];
    mark_done_task(target_title.clone())?;

    Ok(())
}
