use crate::RplcError;
use std::fs;

use super::{find_data_file, get_filename};

pub fn run(args: Vec<String>) -> Result<(), RplcError> {
    let filename = get_filename(args)?;
    let data_file = find_data_file(&filename)?;

    match fs::copy(&filename, data_file) {
        Ok(_) => {
            println!("added '{}' to memory.", filename);
            Ok(())
        }
        Err(err) => {
            println!("failed to add '{}' to memory.", filename);
            Err(RplcError::Other(err.to_string()))
        }
    }
}
