use std::convert::TryInto;
use std::env::current_exe;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;

use chrono::offset::Local;
use chrono::Date;
use chrono::TimeZone;

use crate::command::types::{label::Label, task::Task};

fn load_task_json() -> Result<Vec<Task>, std::io::Error> {
    let dir_path = &mut current_exe()?;
    dir_path.push("task.json");

    let file = File::open(dir_path)?;

    let tasks_json: Vec<Task> = serde_json::from_reader(file)?;

    Ok(tasks_json)
}

fn create_task(title: &str, label: Option<Vec<&str>>, limit: Option<u64>) -> Result<(), String> {
    Ok(())
}

fn create_label(title: &str) -> Result<(), String> {
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
