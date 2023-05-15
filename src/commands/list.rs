use std::fs;

use crate::RplcError;

use super::find_data_file;

pub fn run() -> Result<(), RplcError> {
    let data_dir = find_data_file("")?;

    match fs::read_dir(data_dir) {
        Ok(mut dir) => {
            println!("the following items are currently in memory:");
            while let Some(file) = dir.next() {
                match file {
                    Ok(file) => {
                        println!("{:?}", file.file_name());
                    }
                    Err(err) => {
                        eprintln!("couldn't read memory item: {:?}", err);
                    }
                }
            }

            Ok(())
        }
        Err(err) => {
            println!("failed to list memory.");
            Err(RplcError::Other(err.to_string()))
        }
    }
}
