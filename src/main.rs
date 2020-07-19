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

fn create_data_json() -> Result<(), String> {
    create_file("tasks.json").map_err(|err| err.to_string())?;
    create_file("labels.json").map_err(|err| err.to_string())?;

    Ok(())
}

fn execute(args: Vec<String>) -> Result<(), String> {
    create_data_json()?;

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
            return Err("unknown command".to_string());
        }
    }

    Ok(())
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args = match config::check_args::check_args(raw_args) {
        Ok(arg) => arg,
        Err(error) => {
            error!("{}", error);
            std::process::exit(1);
        }
    };

    if let Err(error) = execute(args) {
        error!("{}", error);
        std::process::exit(1);
    }
}
