pub fn check_args(args: Vec<String>) -> Result<Vec<String>, String> {
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
