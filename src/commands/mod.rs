pub mod save;

#[derive(Debug)]
pub enum ExecutionError {
    MissingArguments(String),
    WrongArguments(String, String),
    Other(String),
}
