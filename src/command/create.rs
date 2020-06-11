use chrono::offset::Local;
use chrono::DateTime;
use chrono::TimeZone;

use std::fs::File;

use crate::command::types::{label::Label, task::Task};

fn load_json() {}

fn create_task(
    title: &str,
    label: Option<Vec<&str>>,
    limit: Option<DateTime<Local>>,
) -> Result<(), String> {
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

fn get_limit(args: &Vec<String>) -> Option<DateTime<Local>> {
    if let Some(index) = args.iter().position(|arg| arg == "--limit") {
        if args.len() == index + 1 {
            return None;
        }
        let raw_date_time = &args[index + 1];
        return Local.datetime_from_str(raw_date_time, "%Y-%m-%d").ok();
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
