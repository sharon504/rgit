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
    TagAlreadyExists(String),
    AmbiguousReference(String),
    RefNotFound(String),
    TagNotFound(String),
    NotInBranch,
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
            RgitError::TagAlreadyExists(name) => {
                writeln!(f, "Fatal: {name} tag already exists")
            }
            RgitError::AmbiguousReference(name) => {
                writeln!(
                    f,
                    "Fatal: '{}' is both a branch and a tag. Please use 'refs/heads/{}' or 'refs/tags/{}'",
                    name, name, name
                )
            }
            RgitError::RefNotFound(name) => {
                writeln!(f, "Fatal: reference '{}' not found", name)
            }
            RgitError::TagNotFound(name) => {
                writeln!(f, "Fatal: tag {name} not found")
            }
            RgitError::NotInBranch => {
                writeln!(f, "Fatal: cannot commit in a non-branch state")
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
