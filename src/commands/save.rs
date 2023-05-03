use crate::RplcError;
use std::{env, fs};
use xdg::BaseDirectories;

use super::get_filename;

pub fn run(args: Vec<String>) -> Result<(), RplcError> {
    let filename = get_filename(args)?;

    let path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => return Err(RplcError::Other(err.to_string())),
    };
    let file_path = format!("{}/{}", path.display(), filename);

    let dirs = match BaseDirectories::new() {
        Ok(dirs) => dirs,
        Err(err) => {
            return Err(RplcError::Other(format!(
                "couldn't read xdg directories: {}",
                err.to_string()
            )))
        }
    };
    let data_path = match dirs.create_data_directory("rplc") {
        Ok(data_path) => format!("{}/{}", data_path.as_path().display().to_string(), filename),
        Err(err) => {
            return Err(RplcError::Other(format!(
                "couldn't validate data directory: {}",
                err
            )))
        }
    };

    match fs::copy(file_path, data_path) {
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
