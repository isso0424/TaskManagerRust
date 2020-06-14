use std::env;
use std::env::current_exe;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::command::types::{label::Labels, task::Tasks};
#[macro_use]
extern crate log;

mod command;

fn load_args() -> Result<Vec<String>, String> {
    let not_enough_args: String = "Not enough args".to_owned();
    let invalid_command: String = "Invalid subcommand".to_owned();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(not_enough_args.to_owned());
    }

    let command = &args[1];

    match command.as_str() {
        "create" => {
            if args.len() < 4 {
                return Err(not_enough_args);
            }
        }
        "check" => {
            if args.len() < 3 {
                return Err(not_enough_args);
            }
        }
        "done" => {
            if args.len() < 3 {
                return Err(not_enough_args);
            }
        }
        "update" => {
            if args.len() < 4 {
                return Err(not_enough_args);
            }
        }
        "delete" => {
            if args.len() < 4 {
                return Err(not_enough_args);
            }
        }
        _ => return Err(invalid_command),
    }

    Ok(args)
}

fn create_json() -> Result<(), std::io::Error> {
    let task_dir_path = current_exe()?.parent().unwrap().join("tasks.json");

    if !task_dir_path.is_file() {
        let file = File::create(task_dir_path)?;
        let mut writer = BufWriter::new(file);
        let empty_tasks = Tasks { content: vec![] };

        write!(writer, "{}", serde_json::to_string(&empty_tasks)?)?;
    }

    let label_dir_path = current_exe()?.parent().unwrap().join("labels.json");

    if !label_dir_path.is_file() {
        let file = File::create(label_dir_path)?;
        let mut writer = BufWriter::new(file);
        let empty_labels = Labels { content: vec![] };

        write!(writer, "{}", serde_json::to_string(&empty_labels)?)?;
    }

    Ok(())
}

fn execute(args: Vec<String>) -> Result<(), String> {
    let command_name = &args[1];

    match command_name.as_str() {
        "create" => {
            command::create::create(args)?;
        }
        "check" => print!("check"),
        "delete" => print!("delete"),
        "done" => print!("done"),
        "update" => print!("update"),
        _ => {
            error!("Command not found");
            return Err("UNKNOWN ERROR".to_owned());
        }
    }

    Ok(())
}

fn main() {
    let args = match load_args() {
        Ok(arg) => arg,
        Err(error) => {
            error!("{}", error);
            std::process::exit(1);
        }
    };
    let _ = create_json().map_err(|x| error!("{}", x));
    match execute(args) {
        Ok(_) => print!("ok"),
        Err(error) => print!("{}", error),
    }
}
