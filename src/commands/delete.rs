use crate::RplcError;
use std::fs;

use super::{find_data_file, get_filename};

pub fn run(args: Vec<String>) -> Result<(), RplcError> {
    let filename = get_filename(args)?;
    let data_dir = find_data_file("")?;

    match fs::remove_file(&format!("{}/{}", data_dir, filename)) {
        Ok(_) => {
            println!("deleted '{}' from memory.", filename);
            Ok(())
        }
        Err(err) => {
            println!("failed to delete '{}' from mememory.", filename);
            Err(RplcError::Other(err.to_string()))
        }
    }
}
