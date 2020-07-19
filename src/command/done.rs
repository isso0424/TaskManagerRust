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
    let index = tasks.get_index(title)?;

    if tasks.content[index].done {
        return Err("Selected task already done".to_string());
    }

    tasks.content[index] = Task {
        title: task.title.clone(),
        label: task.label.clone(),
        limit: task.limit,
        done: true,
    };

    Ok(tasks)
}

fn mark_done_task(task_title: String) -> Result<(), String> {
    let tasks = Tasks::load().map_err(|err| err.to_string())?;
    let tasks = update_task(tasks, task_title)?;

    tasks.save().map_err(|err| err.to_string())?;

    Ok(())
}

pub fn done(args: Vec<String>) -> Result<(), String> {
    let target_title = args[2].clone();
    mark_done_task(target_title)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn update_task_success() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: false,
            }],
        };

        assert_eq!(
            update_task(tasks.clone(), "target".to_string()).unwrap(),
            Tasks {
                content: vec![Task {
                    title: "target".to_string(),
                    label: None,
                    limit: None,
                    done: true
                }]
            }
        );
    }

    #[test]
    fn update_task_failed() {
        let tasks = Tasks {
            content: vec![Task {
                title: "target".to_string(),
                label: None,
                limit: None,
                done: true,
            }],
        };

        assert_eq!(update_task(tasks.clone(), "".to_string()).ok(), None);
        assert_eq!(update_task(tasks.clone(), "target".to_string()).ok(), None);
    }
}
