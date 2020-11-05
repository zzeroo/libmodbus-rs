use std::io;
use std::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    Client { msg: String, source: io::Error },
    Mapping { msg: String, source: io::Error },
    Rtu { msg: String, source: io::Error },
    Server { msg: String, source: io::Error },
    TcpPi { msg: String, source: io::Error },
    Tcp { msg: String, source: io::Error },
    Modbus { msg: String, source: io::Error },
    IoError(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Client {ref msg, source: _} => write!(f, "Client Error: {:?}", msg),
            Error::Mapping {ref msg, source: _} => write!(f, "Mapping Error: {:?}", msg),
            Error::Rtu {ref msg, source: _} => write!(f, "Rtu Error: {:?}", msg),
            Error::Server {ref msg, source: _} => write!(f, "Server Error: {:?}", msg),
            Error::TcpPi {ref msg, source: _} => write!(f, "TcpPi Error: {:?}", msg),
            Error::Tcp {ref msg, source: _} => write!(f, "Tcp Error: {:?}", msg),
            Error::Modbus {ref msg, source: _} => write!(f, "Modbus Error: {:?}", msg),
            Error::IoError(ref err) => write!(f, "IO Error: {:?}", err),
        }
    }
}

impl From <io::Error> for Error {
    fn from(err: io::Error) -> Error {
        Error::IoError(err)
    }
}


impl std::error::Error for Error {}
