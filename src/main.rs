use std::env;

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

fn main() {
    let args = load_args();
}
