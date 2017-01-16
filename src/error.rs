use std::fmt;


#[derive(Debug, Eq, PartialEq)]
pub enum Error {
    /// Could not connect
    ConnectionError,
    /// Invalid modbus slave id.
    InvalidSlaveID,
    InvalidRTUSerialMode,
    InvalidRTURTS,
    InvalidDebug,
    // Could not write register or coil
    ReadFailure,
    WriteFailure,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::ConnectionError => write!(f, "Could not connect."),
            Error::InvalidSlaveID => write!(f, "Invalid modbus slave id."),
            Error::InvalidRTUSerialMode => write!(f, "Invalid RTU serial mode."),
            Error::InvalidRTURTS => write!(f, "Invalid RTU rts."),
            Error::InvalidDebug => write!(f, "Invalid debug mode, only `true` of `false` are allowed."),
            Error::ReadFailure => write!(f, "Could not read."),
            Error::WriteFailure => write!(f, "Could not write."),
        }
    }
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::ConnectionError => "Could not connect",
            Error::InvalidSlaveID => "Invalid modbus slave id",
            Error::InvalidRTUSerialMode => "Invalid RTU serial mode",
            Error::InvalidRTURTS => "Invalid RTU rts",
            Error::InvalidDebug => "Invalid debug mode",
            Error::ReadFailure => "Could not read this value",
            Error::WriteFailure => "Could not write this value",
        }
    }

    fn cause(&self) -> Option<&::std::error::Error> {
        match *self {
            Error::ConnectionError => None,
            Error::InvalidSlaveID => None,
            Error::InvalidRTUSerialMode => None,
            Error::InvalidRTURTS => None,
            Error::InvalidDebug => None,
            Error::ReadFailure => None,
            Error::WriteFailure => None,
        }
    }
}