use rplc::{parse_args, ParsingError};

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

    println!("{:?}", args);

    Ok(())
}
