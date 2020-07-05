use crate::command::types::task::{Task, Tasks};

fn mark_done_task(task_title: String) -> Result<(), String> {
    let mut tasks = Tasks::load().map_err(|err| err.to_string())?;

    if !tasks.content.iter().any(|task| task.title == task_title) {
        return Err("Task not found".to_string());
    }

    let task = tasks
        .content
        .iter()
        .find(|task| task.title == task_title)
        .unwrap();
    let index = Tasks::get_index(task_title)?;

    tasks.content[index] = Task {
        title: task.title.clone(),
        label: task.label.clone(),
        limit: task.limit,
        done: true,
    };

    tasks.save().map_err(|err| err.to_string())?;

    Ok(())
}

pub fn done(args: Vec<String>) -> Result<(), String> {
    let target_title = &args[2];
    mark_done_task(target_title.clone())?;

    Ok(())
}
