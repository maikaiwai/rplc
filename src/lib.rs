use commands::save;
use std::env;

pub mod commands;

#[derive(Debug)]
pub enum RplcError {
    MissingArguments(String),
    WrongArgument(String, String),
    UnknownCommand(String),
    NoCommand,
    Other(String),
}

#[derive(Debug, Clone)]
pub enum Command {
    Save,
    Spawn,
    List,
    Delete,
    Set,
}

pub struct CommandBuilder {
    pub command: Option<Command>,
    pub args: Vec<String>,
}

impl CommandBuilder {
    pub fn new() -> Self {
        CommandBuilder {
            command: None,
            args: vec![],
        }
    }

    pub fn parse(mut self, mut args: env::Args) -> Result<Self, RplcError> {
        let command = match args.nth(1) {
            Some(arg) if arg == "save" => Command::Save,
            Some(arg) if arg == "spawn" => Command::Spawn,
            Some(arg) if arg == "list" => Command::List,
            Some(arg) if arg == "delete" => Command::Delete,
            Some(arg) if arg == "set" => Command::Set,
            Some(arg) => return Err(RplcError::UnknownCommand(arg)),
            None => return Err(RplcError::NoCommand),
        };

        self.command = Some(command);
        self.args = args.take_while(|_| true).collect();

        Ok(self)
    }

    pub fn run(self) -> Result<(), RplcError> {
        match self.command {
            Some(Command::Save) => save::run(self.args),
            Some(Command::Spawn) => todo!(),
            Some(Command::List) => todo!(),
            Some(Command::Delete) => todo!(),
            Some(Command::Set) => todo!(),
            None => todo!(),
        }
    }
}
