// https://github.com/withoutboats/failure/issues/110
use failure;
use std::io;


#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO Error {}", _0)]
    IoError(#[cause] io::Error),
    #[fail(display = "Error {}", _0)]
    Custom(failure::Error),
}

impl From <io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}

impl From<failure::Error> for Error {
    fn from(err: failure::Error) -> Error {
        Error::Custom(err)
    }
}
