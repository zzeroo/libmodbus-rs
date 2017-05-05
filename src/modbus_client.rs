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
use errors::*;
use libc::{c_char, c_int};
use libmodbus_sys;
use modbus::Modbus;
use std::ffi::CString;
use std::io::Error;
use std::str;

pub trait ModbusClient {
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
    fn read_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>>;

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
    fn read_input_bits(&self, addrress: c_int, num_bit: c_int) -> Result<Vec<u8>>;

    /// `read_registers` - read many registers
    ///
    /// The [`read_registers()`](#method.read_registers) function shall read the content of the nb holding registers to the address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x03 (read holding registers).
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
    fn read_registers(&self, address: c_int, num_bits: c_int) -> Result<Vec<u16>>;

    /// `read_input_registers` -  read many input registers
    ///
    /// The [`read_input_registers()`](#method.read_input_registers) function shall read the content of the nb input registers to address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x04 (read input registers). The holding registers and input registers have different historical meaning,
    /// but nowadays it’s more common to use holding registers only.
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
    fn read_input_registers(&self, address: c_int, num_bits: c_int) -> Result<Vec<u16>>;

    /// `report_slave_id` - returns a description of the controller
    ///
    /// The [`report_slave_id()`](#method.report_slave_id) function shall send a request to the controller to obtain a description of the controller.
    ///
    /// The response stored in dest contains:
    ///     * the slave ID, this unique ID is in reality not unique at all so it's not possible to depend on it to know how the
    ///     information are packed in the response.
    ///     * the run indicator status (0x00 = OFF, 0xFF = ON)
    ///     * additional data specific to each controller. For example, libmodbus returns the version of the library as a string.
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
    fn report_slave_id(&self, max_dest: c_int) -> Result<Vec<u8>>;

    /// `write_bit` - read many registers
    ///
    /// The [`write_bit()`](#method.write_bit) function shall write the status of status at the address addr of the remote device.
    /// The value must be set to `true` of `false`.
    ///
    /// The function uses the Modbus function code 0x05 (force single coil).
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
    fn write_bit(&self, address: c_int, status: bool) -> Result<()>;

    fn write_bits(&self, address: c_int, num_bit: c_int, src: Vec<u8>) -> Result<()>;

    fn write_register(&self, address: c_int, value: c_int) -> Result<()>;

    fn write_registers(&self, address: c_int, num_bit: c_int, src: Vec<u16>) -> Result<()>;

    /// `write_and_read_registers` - write and read many registers in a single transaction
    ///
    /// The modbus_write_and_read_registers() function shall write the content of the write_nb holding registers from the array src to the address write_addr of the remote device then shall read the content of the read_nb holding registers to the address read_addr of the remote device. The result of reading is stored in dest array as word values (16 bits).
    ///
    /// The function uses the Modbus function code 0x17 (write/read registers).
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
    fn write_and_read_registers(&self, write_address: c_int, write_num_bit: c_int, src: Vec<u16>,
                                       read_address: c_int, read_num_bit: c_int, mut dest: Vec<u16>) -> Result<()>;

    /// `send_raw_request` - send a raw request
    ///
    /// The [`send_raw_request()`](#method.send_raw_request) function shall send a request via the socket of the context ctx.
    /// This function must be used for debugging purposes because you have to take care to make a valid request by hand.
    /// The function only adds to the message, the header or CRC of the selected backend, so raw_req must start and contain at least a slave/unit identifier and a function code.
    /// This function can be used to send request not handled by the library.
    ///
    /// The public header of libmodbus provides a list of supported Modbus functions codes,
    /// prefixed by MODBUS_FC_ (eg. MODBUS_FC_READ_HOLDING_REGISTERS), to help build of raw requests.
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
    fn send_raw_request(&self, raw_request: Vec<u8>) -> Result<i32>;

    /// `receive_confirmation` - receive a confirmation request
    /// The [`receive_confirmation()`](#method.receive_confirmation) function shall receive a request via the socket of the context ctx.
    /// This function must be used for debugging purposes because the received response isn’t checked against the initial request.
    /// This function can be used to receive request not handled by the library.
    ///
    /// The maximum size of the response depends on the used backend,
    /// in RTU the rsp array must be MODBUS_RTU_MAX_ADU_LENGTH bytes and in TCP it must be MODBUS_TCP_MAX_ADU_LENGTH bytes.
    /// If you want to write code compatible with both, you can use the constant MODBUS_MAX_ADU_LENGTH (maximum value of all libmodbus backends).
    /// Take care to allocate enough memory to store responses to avoid crashes of your server.
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
    fn receive_confirmation(&self) -> Result<Vec<u8>>;

}


impl ModbusClient for Modbus {
    fn read_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>> {
        unsafe {
            let mut tab_reg = vec![0u8; num_bit as usize];
            match libmodbus_sys::modbus_read_bits(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err("Could not read bits".into()) }
                 _ => { Ok(tab_reg) }
            }
        }
    }

    fn read_input_bits(&self, address: c_int, num_bit: c_int) -> Result<Vec<u8>> {
        unsafe {
            let mut tab_reg = vec![0u8; num_bit as usize];
            match libmodbus_sys::modbus_read_input_bits(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err("Could not read input bits".into()) }
                 _ => { Ok(tab_reg) }
            }
        }
    }

    fn read_registers(&self, address: c_int, num_bit: c_int) -> Result<Vec<u16>> {
        unsafe {
            let mut tab_reg = vec![0u16; num_bit as usize];
            match libmodbus_sys::modbus_read_registers(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err("Could not read registers".into()) }
                 _ => { Ok(tab_reg) }
            }
        }
    }

    fn read_input_registers(&self, address: c_int, num_bit: c_int) -> Result<Vec<u16>> {
        unsafe {
            let mut tab_reg = vec![0u16; num_bit as usize];
            match libmodbus_sys::modbus_read_input_registers(self.ctx, address, num_bit, tab_reg.as_mut_ptr()){
                -1 => { Err("Could not read registers".into()) }
                 _ => { Ok(tab_reg) }
            }
        }
    }

    fn report_slave_id(&self, max_dest: c_int) -> Result<Vec<u8>> {
        unsafe {
            let mut tab_reg = vec![0u8; max_dest as usize];
            match libmodbus_sys::modbus_report_slave_id(self.ctx, max_dest, tab_reg.as_mut_ptr()){
                -1 => { Err("Could not report slave id".into()) }
                 _ => { Ok(tab_reg) }
            }
        }
    }

    fn write_bit(&self, address: c_int, status: bool) -> Result<()> {
        unsafe {
            match libmodbus_sys::modbus_write_bit(self.ctx, address, status as i32){
                -1 => { Err("Could not write bit".into()) }
                 _ => { Ok(()) }
            }
        }
    }

    fn write_bits(&self, address: c_int, num_bit: c_int, src: Vec<u8>) -> Result<()> {
        unsafe {
            match libmodbus_sys::modbus_write_bits(self.ctx, address, num_bit, src.as_ptr()){
                -1 => { Err("Could not write bits".into()) }
                 _ => { Ok(()) }
            }
        }
    }

    fn write_register(&self, address: c_int, value: c_int) -> Result<()> {
        unsafe {
            match libmodbus_sys::modbus_write_register(self.ctx, address, value as i32) {
                -1 => { Err("Could not write register".into()) }
                 _ => { Ok(()) }
            }
        }
    }

    fn write_registers(&self, address: c_int, num_bit: c_int, src: Vec<u16>) -> Result<()> {
        unsafe {
            match libmodbus_sys::modbus_write_registers(self.ctx, address, num_bit, src.as_ptr()) {
                -1 => { Err("Could not write registers".into()) }
                 _ => { Ok(()) }
            }
        }
    }

    fn write_and_read_registers(&self, write_address: c_int, write_num_bit: c_int, src: Vec<u16>,
                                       read_address: c_int, read_num_bit: c_int, mut dest: Vec<u16>) -> Result<()> {
        unsafe {
            match libmodbus_sys::modbus_write_and_read_registers(self.ctx, write_address, write_num_bit, src.as_ptr(),
                                                                    read_address, read_num_bit, dest.as_mut_ptr()) {
                -1 => { Err("Could not write and read registers".into() ) }
                 _ => { Ok(()) }
             }
        }
    }

    fn send_raw_request(&self, mut raw_request: Vec<u8>) -> Result<i32> {
        unsafe {
            match libmodbus_sys::modbus_send_raw_request(self.ctx, raw_request.as_mut_ptr(), raw_request.len() as i32) {
                -1 => { Err("Could not send raw request".into()) }
                num => { Ok(num) }
            }
        }
    }

    fn receive_confirmation(&self) -> Result<Vec<u8>> {
        unimplemented!()
    }


}
