use crate::std_result::StdResult;

#[derive(Debug, PartialEq)]
pub enum Command {
    Quit,
    GetBlockchainInfo,
}

// Get command from console
pub fn read_command() -> String {
    let mut line = String::new();
    std::io::stdin()
        .read_line(&mut line)
        .expect("Failed to read from command line");
    line
}

// From user string to command
pub fn translate_command(command: &str) -> StdResult<Command> {
    let normalized_command = command.trim().to_lowercase();

    match normalized_command.as_str() {
        "quit" => Ok(Command::Quit),
        "q" => Ok(Command::Quit),
        "getblockchaininfo" => Ok(Command::GetBlockchainInfo),
        _ => Err("invalid command".into()),
    }
}

#[cfg(test)]
mod command_line_tests {
    use crate::command_line::{translate_command, Command};

    #[test]
    fn unknown_command() {
        let command = translate_command("unknown");
        assert_eq!(command.unwrap_err().to_string(), "invalid command");
    }

    #[test]
    fn correct_quit() {
        let command = translate_command("quit");
        assert_eq!(command.unwrap(), Command::Quit);
    }

    #[test]
    fn correct_getblockchaininfo() {
        let command = translate_command("getblockchaininfo");
        assert_eq!(command.unwrap(), Command::GetBlockchainInfo);
    }

    #[test]
    fn correct_non_trimmed_quit() {
        let command = translate_command(" Quit  ");
        assert_eq!(command.unwrap(), Command::Quit);
    }
}
