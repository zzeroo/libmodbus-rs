use failure::{Backtrace, Context, Error, Fail};
use std::fmt;
use std::fmt::Display;


#[derive(Debug)]
pub struct ModbusError {
    inner: Context<ModbusErrorKind>,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Fail)]
pub enum ModbusErrorKind {
    #[fail(display = "Function code received in the query is not recognized or allowed by slave")]
    IllegalFunction,
    //  {
    //     display("Illegal function")
    // }
    #[fail(display = "Data address of some or all the required entities are not allowed or do not exist in slave")]
    IllegalDataAddress,
    //  {
    //     display("Illegal data address")
    // }
    #[fail(display = "Value is not accepted by slave")]
    IllegalDataValue,
    //  {
    //     display("Illegal data value")
    // }
    #[fail(display = "Unrecoverable error occurred while slave was attempting to perform requested action")]
    SlaveDeviceOrServerFailure,
    //  {
    //     display("Slave device or server failure")
    // }
    #[fail(display = "Acknowledge")]
    Acknowledge,
    //  {
    //     display("Acknowledge")
    // }
    #[fail(display = "Slave device or server is busy")]
    SlaveDeviceOrServerIsBusy,
    //  {
    //     display("Slave device or server is busy")
    // }
    #[fail(display = "Negative acknowledge")]
    NegativeAcknowledge,
    //  {
    //     display("Negative acknowledge")
    // }
    #[fail(display = "Memory parity error")]
    MemoryParityError,
    //  {
    //     display("Memory parity error")
    // }
    #[fail(display = "Gateway path unavailable")]
    GatewayPathUnavailable,
    //  {
    //     display("Gateway path unavailable")
    // }
    #[fail(display = "Target device failed to respond")]
    TargetDeviceFailedToRespond,
    //  {
    //     display("Target device failed to respond")
    // }
    #[fail(display = "Invalid CRC")]
    InvalidCRC,
    //  {
    //     display("Invalid CRC")
    // }
    #[fail(display = "Invalid data")]
    InvalidData,
    //  {
    //     display("Invalid data")
    // }
    #[fail(display = "Invalid exception code")]
    InvalidExceptionCode,
    //  {
    //     display("Invalid exception code")
    // }
    #[fail(display = "Too many data requested")]
    TooManyData,
    //  {
    //     display("Too many data")
    // }
    #[fail(display = "Response not from requested slave")]
    ResponseNotFromRequestedSlave,
    //  {
    //     display("Response not from requested slave")
    // }
    #[fail(display = "Response not from requested slave")]
    BADSLAVE,
    //  {
    //     display("Response not from requested slave")
    // }
    #[fail(display = "libmodbus API incompatible response")]
    IncompatibleAPI,
    //  {
    //     display("libmodbus API incompatible response")
    // }
    #[fail(display = "Invalid slave ID")]
    InvalidSlaveID,
    //  {
    //     display("Invalid slave ID: '{}'", id)
    // }
    #[fail(display = "Invalid parameter given")]
    InvalidParameter,
    //  {
    //     display("Invalid parameter given: '{}'", desc)
    // }
    #[fail(display = "Unit test client failed")]
    UnitTestClientFailure,
    //  {
    //     display("Unit test client failed")
    // }
}

impl Fail for ModbusError {
    fn cause(&self) -> Option<&Fail> {
        self.inner.cause()
    }

    fn backtrace(&self) -> Option<&Backtrace> {
        self.inner.backtrace()
    }
}

impl Display for ModbusError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        Display::fmt(&self.inner, f)
    }
}

impl ModbusError {
    pub fn kind(&self) -> ModbusErrorKind {
        *self.inner.get_context()
    }
}

impl From<ModbusErrorKind> for ModbusError {
    fn from(kind: ModbusErrorKind) -> ModbusError {
        ModbusError { inner: Context::new(kind) }
    }
}

impl From<Context<ModbusErrorKind>> for ModbusError {
    fn from(inner: Context<ModbusErrorKind>) -> ModbusError {
        ModbusError { inner: inner }
    }
}
