use std::convert::TryInto;
use std::env::current_exe;
use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Write;
use std::path::PathBuf;

use chrono::offset::Local;
use chrono::TimeZone;

use crate::command::types::{
    label::{Label, Labels},
    task::{Task, Tasks},
};

fn save(json: String, dir_path: PathBuf) -> Result<(), std::io::Error> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(dir_path)?;

    let mut writer = BufWriter::new(file);

    write!(writer, "{}", json)?;

    Ok(())
}

fn load_label_json() -> Result<Labels, std::io::Error> {
    let mut dir_path = current_exe()?;
    dir_path.push("labels.json");

    let file = File::open(dir_path)?;

    let label_json = serde_json::from_reader(file)?;

    Ok(label_json)
}

fn save_label_json(labels_vec: Vec<Label>) -> Result<(), std::io::Error> {
    let mut dir_path = current_exe()?;
    dir_path.push("labels.json");

    let labels = Labels {
        content: labels_vec,
    };

    let json = serde_json::to_string(&labels)?;

    save(json, dir_path)?;

    Ok(())
}

fn create_label(title: &str) -> Result<(), String> {
    let mut labels = match load_label_json() {
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

    save_label_json(labels.content).map_err(|err| return err.to_string())?;

    Ok(())
}

fn load_task_json() -> Result<Tasks, std::io::Error> {
    let dir_path = &mut current_exe()?;
    dir_path.push("tasks.json");

    let file = File::open(dir_path)?;

    let tasks_json = serde_json::from_reader(file)?;

    Ok(tasks_json)
}

fn save_task_json(tasks_vec: Vec<Task>) -> Result<(), std::io::Error> {
    let mut dir_path = current_exe()?;
    dir_path.push("tasks.json");

    let tasks = Tasks { content: tasks_vec };

    let json = serde_json::to_string(&tasks)?;

    save(json, dir_path)?;

    Ok(())
}

fn parse_to_label(raw_labels: Option<Vec<&str>>) -> Result<Option<Vec<Label>>, String> {
    let labels = &mut vec![];
    if raw_labels.is_none() {
        return Ok(None);
    }
    let all_labels = &load_label_json().map_err(|err| return err.to_string())?;
    for raw_label in match raw_labels {
        Some(value) => value,
        None => return Ok(None),
    } {
        if !all_labels
            .content
            .iter()
            .any(|label| label.title.to_string() == raw_label)
        {
            continue;
        }
        labels.push(Label {
            title: raw_label.to_string(),
        });
    }

    Ok(Some(labels.clone()))
}

fn create_task(title: &str, label: Option<Vec<&str>>, limit: Option<u64>) -> Result<(), String> {
    let mut tasks = match load_task_json() {
        Ok(tasks) => tasks,
        Err(err) => return Err(err.to_string()),
    };

    if tasks.content.iter().any(|x| x.title == title) {
        return Err("Cannot use duplicate task title".to_string());
    }

    let new_task = Task {
        title: title.to_string(),
        label: parse_to_label(label)?,
        limit: limit,
    };

    tasks.content.push(new_task);

    let _ = save_task_json(tasks.content).map_err(|err| return err.to_string())?;

    Ok(())
}

fn get_label<'a>(args: &'a Vec<String>) -> Option<Vec<&'a str>> {
    if let Some(index) = args.iter().position(|arg| arg == "--label") {
        if args.len() == index + 1 {
            return None;
        }
        let labels = args[index + 1].split(",").collect();
        return Some(labels);
    }
    return None;
}

fn get_limit(args: &Vec<String>) -> Option<u64> {
    if let Some(index) = args.iter().position(|arg| arg == "--limit") {
        if args.len() == index + 1 {
            return None;
        }
        let raw_date_time = &args[index + 1];
        let date_time = Local.datetime_from_str(raw_date_time, "%Y-%m-%d").ok();
        if date_time.is_none() {
            return None;
        }
        return date_time?.timestamp().try_into().ok();
    }
    return None;
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
