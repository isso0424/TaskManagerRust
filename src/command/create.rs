use crate::command::types::{
    label::{Label, Labels},
    task::{Task, Tasks},
};

use crate::config::parse_arg::{get_label, get_limit};

fn create_label(title: &str) -> Result<(), String> {
    let mut labels = match Labels::load() {
        Ok(labels) => labels,
        Err(err) => return Err(err.to_string()),
    };

    if labels.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate label title".to_string());
    }

    let new_label = Label {
        title: title.to_string(),
    };

    labels.content.push(new_label);

    let _ = labels.save().map_err(|err| return err.to_string())?;

    Ok(())
}

fn create_task(title: &str, label: Option<Vec<&str>>, limit: Option<u64>) -> Result<(), String> {
    let mut tasks = match Tasks::load() {
        Ok(tasks) => tasks,
        Err(err) => return Err(err.to_string()),
    };

    if tasks.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate task title".to_string());
    }

    let new_task = Task {
        title: title.to_string(),
        label: Labels::parse(label)?,
        limit: limit,
    };

    tasks.content.push(new_task);

    let _ = tasks.save().map_err(|err| return err.to_string())?;

    Ok(())
}

pub fn create(args: Vec<String>) -> Result<(), String> {
    let invalid_target = "Invalid Target".to_owned();
    print!("create");
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
