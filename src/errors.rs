use std::fmt;
use std::io;
// use std::error::Error;
// use std::path::PathBuf;

#[derive(Debug)]
pub enum RgitError {
    Io(io::Error),
    NotInitialized(String),
    // CreatingFolder(PathBuf, Box<dyn Error>),
    MessageFileNotFound,
    // RootFolderCreation(Box<dyn Error>),
}

impl fmt::Display for RgitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgitError::Io(err) => write!(f, "IO Error: {}", err),
            RgitError::NotInitialized(s) => {
                write!(f, "Fatal: Not a rgit repository({} not found)", s)
            }
            // RgitError::CreatingFolder(dir, err) => {
            //     write!(
            //         f,
            //         "Fatal: Cannot create directory: {:?}\nError: {}",
            //         dir, err
            //     )
            // }
            // RgitError::RootFolderCreation(err) => {
            //     write!(f, "Fatal: Cannot create .rgit directory\nError: {}", err)
            // }
            RgitError::MessageFileNotFound => {
                write!(f, "Fatal: Message file not found")
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
