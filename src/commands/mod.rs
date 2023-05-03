use xdg::BaseDirectories;

use crate::RplcError;

pub mod save;
pub mod spawn;

pub fn get_filename(args: Vec<String>) -> Result<String, RplcError> {
    match args.first() {
        Some(filename) if filename == "." => {
            return Err(RplcError::WrongArgument(
                String::from("filename"),
                String::from("not a directory"),
            ))
        }
        Some(filename) => Ok(filename.clone()),
        None => return Err(RplcError::MissingArguments(String::from("filename"))),
    }
}

pub fn find_data_file(filename: &str) -> Result<String, RplcError> {
    let dirs = match BaseDirectories::new() {
        Ok(dirs) => dirs,
        Err(err) => {
            return Err(RplcError::Other(format!(
                "couldn't read xdg directories: {}",
                err.to_string()
            )))
        }
    };
    match dirs.create_data_directory("rplc") {
        Ok(data_path) => Ok(format!(
            "{}/{}",
            data_path.as_path().display().to_string(),
            filename
        )),
        Err(err) => {
            return Err(RplcError::Other(format!(
                "couldn't validate data directory: {}",
                err
            )))
        }
    }
}
