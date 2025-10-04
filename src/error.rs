use std::io;

#[derive(Debug)]
pub enum BalSaveError {
    Io(io::Error),
    Clap(clap::Error),
    Path(String),
    Validation(String),
    UserCancellation,
}

impl std::error::Error for BalSaveError {}

impl std::fmt::Display for BalSaveError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BalSaveError::Io(err) => write!(f, "I/O Error: {}", err),
            BalSaveError::Clap(err) => write!(f, "Argument Error: {}", err),
            BalSaveError::Path(msg) => write!(f, "Pathing Error: {}", msg),
            BalSaveError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            BalSaveError::UserCancellation => write!(f, "Operation cancelled by user."),
        }
    }
}

impl From<io::Error> for BalSaveError {
    fn from(err: io::Error) -> Self {
        BalSaveError::Io(err)
    }
}

impl From<clap::Error> for BalSaveError {
    fn from(err: clap::Error) -> Self {
        BalSaveError::Clap(err)
    }
}