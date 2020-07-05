use std::env;
use std::env::current_exe;

use std::fs::File;
use std::io::BufWriter;
use std::io::Write;

use crate::command::types::task::Tasks;
#[macro_use]
extern crate log;

mod command;
mod config;

fn load_args() -> Result<Vec<String>, String> {
    let not_enough_args: String = "Not enough args".to_owned();
    let invalid_command: String = "Invalid subcommand".to_owned();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(not_enough_args.to_owned());
    }

    let command = &args[1];

    match command.as_str() {
        "create" | "update" | "delete" => {
            if args.len() < 4 {
                return Err(not_enough_args);
            }
        }
        "check" | "done" => {
            if args.len() < 3 {
                return Err(not_enough_args);
            }
        }
        _ => return Err(invalid_command),
    }

    Ok(args)
}

fn create_file(file_name: &str) -> Result<(), std::io::Error> {
    let path = current_exe()?.parent().unwrap().join(file_name);

    if path.is_file() {
        return Ok(());
    }

    let file = File::create(path)?;
    let mut writer = BufWriter::new(file);
    let empty_tasks = Tasks { content: vec![] };

    write!(writer, "{}", serde_json::to_string(&empty_tasks)?)?;

    Ok(())
}

fn create_data_json() -> Result<(), std::io::Error> {
    create_file("tasks.json")?;
    create_file("labels.json")?;

    Ok(())
}

fn execute(args: Vec<String>) -> Result<(), String> {
    create_data_json().map_err(|err| err.to_string())?;

    let command_name = &args[1];

    match command_name.as_str() {
        "create" => {
            command::create::create(args)?;
        }
        "check" => command::check::check(args)?,
        "delete" => command::delete::delete(args)?,
        "done" => command::done::done(args)?,
        "update" => command::update::update(args)?,
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

    match execute(args) {
        Ok(_) => print!("executed!!!"),
        Err(error) => print!("{}", error),
    }
}
