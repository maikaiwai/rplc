use std::fs;

use crate::RplcError;

use super::{find_data_file, get_filename};

pub fn run(args: Vec<String>) -> Result<(), RplcError> {
    let filename = get_filename(args)?;
    let data_file = find_data_file(&filename)?;

    match fs::copy(data_file, &filename) {
        Ok(_) => {
            println!("spawned copy of {}.", filename);
            Ok(())
        }
        Err(err) => {
            println!("failed to spawn '{}'.", filename);
            Err(RplcError::Other(err.to_string()))
        }
    }
}
