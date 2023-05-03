use std::env;

use rplc::{CommandBuilder, RplcError};

fn main() -> Result<(), RplcError> {
    let command = CommandBuilder::new().parse(env::args())?;

    if let Err(err) = command.run() {
        match err {
            RplcError::MissingArguments(arg) => println!("'{}' is missing.", arg),
            RplcError::WrongArgument(arg, expected) => {
                println!("expected '{}' to be {}", arg, expected)
            }
            RplcError::UnknownCommand(name) => println!("'{}' is not a command.", name),
            RplcError::NoCommand => println!("expected a command name."),
            RplcError::Other(message) => println!("{}", message),
        }
    };

    Ok(())
}
