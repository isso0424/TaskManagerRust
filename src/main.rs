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

fn check_args(args: Vec<String>) -> Result<Vec<String>, String> {
    let not_enough_args: String = "Not enough args".to_string();

    if args.len() < 2 {
        return Err(not_enough_args);
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
        _ => return Err("Invalid subcommand".to_string()),
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
            return Err("unknown command".to_string());
        }
    }

    Ok(())
}

fn main() {
    let raw_args: Vec<String> = env::args().collect();
    let args = match check_args(raw_args) {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_args_success() {
        let commands: Vec<String> = vec!["check", "done", "create", "update", "delete"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();

        let mut args: Vec<String> = vec!["", "command", "hoge"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();

        let switch_timing = 1;
        let mut index = 0;

        for command in commands {
            if switch_timing == index {
                args.push("fuga".to_string())
            }
            args[1] = command;
            assert_eq!(check_args(args.clone()), Ok(args.clone()));
            index += 1;
        }
    }

    #[test]
    fn check_args_failed() {
        let not_enough_args: String = "Not enough args".to_string();

        let commands: Vec<String> = vec!["check", "done", "create", "update", "delete"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();

        let mut args: Vec<String> = vec!["", "command"]
            .iter()
            .map(|arg| arg.to_string())
            .collect();

        let switch_timing = 1;
        let mut index = 0;

        for command in commands {
            args[1] = command;
            assert_eq!(check_args(args.clone()), Err(not_enough_args.clone()));
            if switch_timing == index {
                args.push("fuga".to_string())
            }
            index += 1;
        }

        args[1] = "failed".to_string();

        assert_eq!(
            check_args(args.clone()),
            Err("Invalid subcommand".to_string())
        );
    }
}
