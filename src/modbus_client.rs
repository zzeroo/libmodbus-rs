//! The Modbus protocol defines different data types and functions to read and write them from/to remote devices.
//! The following functions are used by the clients to send Modbus requests:
//!
//! * Read data
//!     - [`read_bits(3)`](#method.read_bits) [`read_input_bits(3)`](#method.read_input_bits) [`read_registers(3)`](#method.read_registers) [`read_input_registers(3)`](#method.read_input_registers) [`report_slave_id(3)`](#method.report_slave_id)
//! * Write data
//!     - [`write_bit(3)`](#method.write_bit) [`write_register(3)`](#method.write_register) [`write_bits(3)`](#method.write_bits) [`write_registers(3)`](#method.write_registers)
//! * Write and read data
//!     - [`write_and_read_registers(3)`](#method.write_and_read_registers)
//! * Raw requests
//!     - [`send_raw_request(3)`](#method.send_raw_request) [`receive_confirmation(3)`](#method.receive_confirmation)
//! * Reply an exception
//!     - [`reply_exception(3)`](#method.reply_exception)
use error::ModbusError;
use libc::{c_char, c_int};
use libmodbus_sys;
use modbus::Modbus;
use std::ffi::CString;
use std::io::Error;
use std::str;

pub trait ModbusClient {
    fn read_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>, ModbusError>;
    fn read_input_bits(&self, addrress: c_int, num_bit: c_int) -> Result<Vec<u8>, ModbusError>;
}

impl ModbusClient for Modbus {
    /// `read_bits` - read many bits
    ///
    /// The [`read_bits()`]() function shall read the status of the nb bits (coils) to the address addr of the remote device.
    /// The result of reading is stored in dest array as unsigned bytes (8 bits) set to TRUE or FALSE.
    ///
    /// The function uses the Modbus function code 0x01 (read coil status).
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {}
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    fn read_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>, ModbusError> {
        unsafe {
            let mut tab_reg = vec![0u8; num_bit as usize];
            match libmodbus_sys::modbus_read_bits(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err(ModbusError::ToManyBits)}
                 _ => { Ok(tab_reg) }
            }
        }
    }

    /// `read_input_bits` - read many input bits
    ///
    /// The [`read_input_bits()`](#method.read_input_bits) function shall read the content of the nb input bits to the address addr of the remote device.
    /// The result of reading is stored in dest array as unsigned bytes (8 bits) set to TRUE or FALSE.
    ///
    /// The function uses the Modbus function code 0x02 (read input status).
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {}
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    fn read_input_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>, ModbusError> {
        unsafe {
            let mut tab_reg = vec![0u8; num_bit as usize];
            match libmodbus_sys::modbus_read_input_bits(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err(ModbusError::ToManyBits)}
                 _ => { Ok(tab_reg) }
            }
        }
    }

}
