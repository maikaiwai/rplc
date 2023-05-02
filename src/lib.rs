#[derive(Debug)]
pub struct RplcArguments {
    pub command: Command,
    pub rest: Vec<String>,
}

#[derive(Debug)]
pub enum Command {
    Save,
    Spawn,
    List,
    Delete,
    Set,
}

pub enum ParsingError {
    NoCommand,
    UnknownCommand,
}

pub fn parse_args() -> Result<RplcArguments, ParsingError> {
    use std::env;

    let mut args = env::args();

    let command = match args.nth(1) {
        Some(arg) if arg == "save" => Command::Save,
        Some(arg) if arg == "spawn" => Command::Spawn,
        Some(arg) if arg == "list" => Command::List,
        Some(arg) if arg == "delete" => Command::Delete,
        Some(arg) if arg == "set" => Command::Set,
        Some(_) => return Err(ParsingError::UnknownCommand),
        None => return Err(ParsingError::NoCommand),
    };

    Ok(RplcArguments {
        command,
        rest: args.take_while(|_| true).collect(),
    })
}
