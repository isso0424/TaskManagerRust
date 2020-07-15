use crate::command::types::{
    label::{Label, Labels},
    task::{Task, Tasks},
};

use crate::config::parse_arg::{get_label, get_limit};

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

fn update_task(
    mut tasks: Tasks,
    title: &str,
    label: Option<Vec<&str>>,
    limit: Option<i64>,
) -> Result<Tasks, String> {
    if tasks.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate task title".to_string());
    }

    let new_task = Task {
        title: title.to_string(),
        label: Labels::parse(label)?,
        limit,
        done: false,
    };

    tasks.content.push(new_task);

    Ok(tasks)
}

fn create_task(title: &str, label: Option<Vec<&str>>, limit: Option<i64>) -> Result<(), String> {
    let tasks = match Tasks::load() {
        Ok(tasks) => update_task(tasks, title, label, limit)?,
        Err(err) => return Err(err.to_string()),
    };
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
