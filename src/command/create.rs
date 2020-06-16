use std::convert::TryInto;
use std::env::current_exe;
use std::fs::File;
use std::path::PathBuf;

use chrono::offset::Local;
use chrono::TimeZone;

use crate::command::types::{
    label::{Label, Labels},
    task::{Task, Tasks},
};

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
        print!("{}", raw_date_time);
        let date_time = Local
            .datetime_from_str(format!("{} 00:00", raw_date_time).as_str(), "%F %R")
            .ok();
        print!("{:?}", date_time);
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
