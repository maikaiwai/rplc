use rplc::{commands::ExecutionError, parse_args, ParsingError};

fn main() -> anyhow::Result<()> {
    let args = match parse_args() {
        Ok(args) => args,
        Err(ParsingError::NoCommand) => {
            println!("missing command.");
            return Ok(());
        }
        Err(ParsingError::UnknownCommand) => {
            println!("unknown command.");
            return Ok(());
        }
    };

    let name = args.command.clone();

    match rplc::run_command(args) {
        Ok(_) => Ok(()),
        Err(ExecutionError::MissingArguments(arg)) => {
            println!("'{:?}' is missing an argument: '{:?}'.", name, arg);
            return Ok(());
        }
        Err(ExecutionError::WrongArguments(arg, expected)) => {
            println!(
                "'{:?}' received the wrong arguments: '{}' expected to be {}.",
                name, arg, expected
            );
            return Ok(());
        }
        Err(ExecutionError::Other(err)) => {
            println!("'{:?}' failed to be executed: {}", name, err);
            return Ok(());
        }
    }
}
