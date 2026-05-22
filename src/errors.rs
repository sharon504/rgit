use std::fmt;
use std::io;

#[derive(Debug)]
pub enum RgitError {
    Io(io::Error),
    NotInitialized(String),
    MessageFileNotFound,
    BranchAlreadyExists(String),
    BranchNotFound(String),
    CommitNotFound,
}

impl fmt::Display for RgitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgitError::Io(err) => writeln!(f, "IO Error: {}", err),
            RgitError::NotInitialized(s) => {
                writeln!(f, "Fatal: Not a rgit repository({} not found)", s)
            }
            RgitError::MessageFileNotFound => {
                writeln!(f, "Fatal: Message file not found")
            }
            RgitError::BranchAlreadyExists(name) => {
                writeln!(f, "Fatal: {name} branch already exists")
            }
            RgitError::BranchNotFound(name) => {
                writeln!(f, "Fatal: branch {name} doesn't exists")
            }
            RgitError::CommitNotFound => {
                writeln!(f, "Fatal: branch commit doesn't exists")
            }
        }
    }
}

impl std::error::Error for RgitError {}

impl From<io::Error> for RgitError {
    fn from(err: io::Error) -> Self {
        RgitError::Io(err)
    }
}
