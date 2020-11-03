use std::io;

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
}

impl From <io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

