use std::env;
#[macro_use]
extern crate log;

mod command;
mod config;

fn execute(args: Vec<String>) -> Result<(), String> {
    config::file::create_data_json()?;

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
