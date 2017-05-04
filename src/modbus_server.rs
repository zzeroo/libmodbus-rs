//! The server is waiting for request from clients and must answer when it is concerned by the request.
//! The libmodbus offers the following functions to handle requests:
//!
use error::ModbusError;
use libc::{c_char, c_int};
use libmodbus_sys;
use modbus::Modbus;
use std::ffi::CString;
use std::io::Error;
use std::str;

pub trait ModbusServer {
}

impl ModbusServer for Modbus {
}
