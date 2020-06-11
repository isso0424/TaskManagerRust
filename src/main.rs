use std::env;
#[macro_use]
extern crate log;

mod command;

fn load_args() -> Result<Vec<String>, String> {
    let not_enough_args: String = "Not enough args".to_owned();
    let invalid_command: String = "Invali subcommand".to_owned();

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

fn execute(args: Vec<String>) -> Result<(), String> {
    let command_name = &args[1];

    match command_name.as_str() {
        "create" => {
            command::create::create(args);
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
    match execute(args) {
        Ok(_) => print!("ok"),
        Err(error) => print!("{}", error),
    }
}
