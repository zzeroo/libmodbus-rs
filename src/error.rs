use std::error::Error;
use std::fmt;
use std::io;


#[derive(Debug)]
pub enum ModbusError {
    InvalArg,
    Io(io::Error),
    NotRTU,
}

impl fmt::Display for ModbusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ModbusError::InvalArg => write!(f, "an invalid argument was given"),
            ModbusError::Io(ref err) => err.fmt(f),
            ModbusError::NotRTU => write!(f, "the libmodbus backend is not RTU"),
        }
    }
}

impl Error for ModbusError {
    fn description(&self) -> &str {
        match *self {
            ModbusError::InvalArg => "an invalid argument was given",
            ModbusError::Io(ref err) => err.description(),
            ModbusError::NotRTU => "the libmodbus backend is not RTU",
        }
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ModbusError::InvalArg => None,
            ModbusError::Io(ref err) => Some(err),
            ModbusError::NotRTU => None,
        }
    }
}

impl From<io::Error> for ModbusError {
    fn from(err: io::Error) -> ModbusError {
        ModbusError::Io(err)
    }
}
