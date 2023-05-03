use crate::RplcError;
use std::{env, fs};
use xdg::BaseDirectories;

pub fn run(args: Vec<String>) -> Result<(), RplcError> {
    let file_name = match args.first() {
        Some(filename) if filename == "." => {
            return Err(RplcError::WrongArgument(
                String::from("filename"),
                String::from("not a directory"),
            ))
        }
        Some(filename) => filename,
        None => return Err(RplcError::MissingArguments(String::from("filename"))),
    };

    let path = match env::current_dir() {
        Ok(path) => path,
        Err(err) => return Err(RplcError::Other(err.to_string())),
    };
    let file_path = format!("{}/{}", path.display(), file_name);

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
        Ok(data_path) => format!(
            "{}/{}",
            data_path.as_path().display().to_string(),
            file_name
        ),
        Err(err) => {
            return Err(RplcError::Other(format!(
                "couldn't validate data directory: {}",
                err
            )))
        }
    };

    match fs::copy(file_path, data_path) {
        Ok(_) => {
            println!("added '{}' to memory.", file_name);
            Ok(())
        }
        Err(err) => {
            println!("failed to add '{}' to memory.", file_name);
            Err(RplcError::Other(err.to_string()))
        }
    }
}
