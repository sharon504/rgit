use std::error::Error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub enum RgitError {
    Io(io::Error),
    NotInitialized,
    CreatingFolder(PathBuf, Box<dyn Error>),
    RootFolderInstallation(Box<dyn Error>),
}

impl fmt::Display for RgitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RgitError::Io(err) => write!(f, "IO Error: {}", err),
            RgitError::NotInitialized => write!(
                f,
                "Fatal: Not a rgit repository (or any of the parent directories): .rgit"
            ),
            RgitError::CreatingFolder(dir, err) => {
                write!(
                    f,
                    "Fatal: Cannot create directory: {:?}\nError: {}",
                    dir, err
                )
            }
            RgitError::RootFolderInstallation(err) => {
                write!(f, "Fatal: Cannot create .rgit directory\nError: {}", err)
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
