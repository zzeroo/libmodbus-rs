use errors::*;
use libc::c_int;
use libmodbus_sys as ffi;
use modbus::Modbus;
use std::io::Error;


/// The Modbus protocol defines different data types and functions to read and write them from/to remote devices.
/// The following functions are used by the clients to send Modbus requests:
///
/// * Read data
///     - [`read_bits()`](struct.Modbus.html#method.read_bits),
/// [`read_input_bits()`](struct.Modbus.html#method.read_input_bits),
/// [`read_registers()`](struct.Modbus.html#method.read_registers),
/// [`read_input_registers()`](struct.Modbus.html#method.read_input_registers),
/// [`report_slave_id()`](struct.Modbus.html#method.report_slave_id)
/// * Write data
///     - [`write_bit()`](struct.Modbus.html#method.write_bit),
/// [`write_register()`](struct.Modbus.html#method.write_register),
/// [`write_bits()`](struct.Modbus.html#method.write_bits),
/// [`write_registers()`](struct.Modbus.html#method.write_registers)
/// * Write and read data
///     - [`write_and_read_registers()`](struct.Modbus.html#method.write_and_read_registers)
/// * Raw requests
///     - [`send_raw_request()`](struct.Modbus.html#method.send_raw_request),
/// [`receive_confirmation()`](struct.Modbus.html#method.receive_confirmation)
/// * Reply an exception
///     - [`reply_exception()`](struct.Modbus.html#method.reply_exception)
///
pub trait ModbusClient {
    fn read_bits(&self, address: u16, num: u16, dest: &mut [u8]) -> Result<u16>;
    fn read_input_bits(&self, address: u16, num: u16, dest: &mut [u8]) -> Result<u16>;
    fn read_registers(&self, address: u16, num: u16, dest: &mut [u16]) -> Result<u16>;
    fn read_input_registers(&self, address: u16, num: u16, dest: &mut [u16]) -> Result<u16>;
    fn report_slave_id(&self, max_dest: usize, dest: &mut [u8]) -> Result<u16>;
    fn write_bit(&self, address: u16, status: bool) -> Result<()>;
    fn write_bits(&self, address: u16, num: u16, src: &[u8]) -> Result<u16>;
    fn write_register(&self, address: u16, value: u16) -> Result<()>;
    fn write_registers(&self, address: u16, num: u16, src: &[u16]) -> Result<u16>;
    fn write_and_read_registers(&self, write_address: u16, write_num: u16, src: &[u16], read_address: u16,
                                read_num: u16, dest: &mut [u16])
                                -> Result<u16>;
    fn mask_write_register(&self, address: u16, and_mask: u16, or_mask: u16) -> Result<()>;
    fn send_raw_request(&self, raw_request: &mut [u8], lenght: i32) -> Result<u16>;
    fn receive_confirmation(&self, response: &mut [u8]) -> Result<u16>;
}

// Convert the given Error (last_os_error()) to a libmodbus Error
//
// TODO: this looks ugly, is there a better way?
fn get_error(error: Error) -> ::errors::Error {
    match error.raw_os_error() {
        Some(112345680) => ErrorKind::IllegalDataAddress.into(),
        Some(112345694) => ErrorKind::TooManyData.into(),
        _ => ErrorKind::IncompatibleAPI.into(),
    }
}

// TODO: add real, working examples
impl ModbusClient for Modbus {
    /// `read_bits` - read many bits
    ///
    /// The [`read_bits()`](#method.read_bits) function shall read the status of the `num` bits (coils) to the
    /// `address` of the remote device. The result of reading is stored in `dest` slice as unsigned bytes (8 bits) set
    /// to TRUE or FALSE.
    ///
    /// The function uses the **Modbus function code 0x01** (read coil status).
    ///
    /// # Return value
    ///
    /// The function returns a `Result` containing the number of read bits if successful. Otherwise it returns an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number of coils to read
    /// * `dest`    - the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut dest = vec![0u8; 100];
    ///
    /// assert!(modbus.read_bits(0, 1, &mut dest).is_ok());
    /// ```
    fn read_bits(&self, address: u16, num: u16, dest: &mut [u8]) -> Result<u16> {
        unsafe {
            match ffi::modbus_read_bits(self.ctx, address as c_int, num as c_int, dest.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }

    /// `read_input_bits` - read many input bits
    ///
    /// The [`read_input_bits()`](#method.read_input_bits) function shall read the content of the `num` input bits to
    /// the `address` of the remote device. The result of reading is stored in `dest` slice as unsigned bytes (8 bits)
    /// set to TRUE or FALSE.
    ///
    /// The function uses the **Modbus function code 0x02** (read input status).
    ///
    /// # Return value
    ///
    /// The function returns a `Result` containing the number of read bits if successful. Otherwise it returns an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number of input bits to read
    /// * `dest`    - the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut dest = vec![0u8; 100];
    ///
    /// assert!(modbus.read_input_bits(0, 1, &mut dest).is_ok());
    /// ```
    fn read_input_bits(&self, address: u16, num: u16, dest: &mut [u8]) -> Result<u16> {
        unsafe {
            match ffi::modbus_read_input_bits(self.ctx, address as c_int, num as c_int, dest.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }

    /// `read_registers` - read many registers
    ///
    /// The [`read_registers()`](#method.read_registers) function shall read the content of the `num` holding registers
    /// to the `address` of the remote device. The result of reading is stored in `dest` slice as u16 word values.
    ///
    /// The function uses the **Modbus function code 0x03** (read holding registers).
    ///
    /// # Return value
    ///
    /// The function returns a `Result` containing the number of read bits if successful. Otherwise it returns an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number of holding registers to read
    /// * `dest`    - the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut dest = vec![0u16; 100];
    ///
    /// assert!(modbus.read_registers(0, 1, &mut dest).is_ok());
    /// ```
    fn read_registers(&self, address: u16, num: u16, dest: &mut [u16]) -> Result<u16> {
        unsafe {
            match ffi::modbus_read_registers(self.ctx, address as c_int, num as c_int, dest.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }

    /// `read_input_registers` -  read many input registers
    ///
    /// The [`read_input_registers()`](#method.read_input_registers) function shall read the content of the `num`
    /// holding registers to the `address` of the remote device. The result of reading is stored in `dest` slice as u16
    /// word values.
    ///
    /// The function uses the **Modbus function code 0x04** (read input registers). The holding registers and input
    /// registers have different historical meaning, but nowadays it’s more common to use holding registers only.
    ///
    /// # Return value
    ///
    /// The function returns a `Result` containing the number of read bits if successful. Otherwise it returns an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number of input registers to read
    /// * `dest`    - the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut dest = vec![0u16; 100];
    ///
    /// assert!(modbus.read_input_registers(0, 1, &mut dest).is_ok());
    /// ```
    fn read_input_registers(&self, address: u16, num: u16, dest: &mut [u16]) -> Result<u16> {
        unsafe {
            match ffi::modbus_read_input_registers(self.ctx, address as c_int, num as c_int, dest.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }

    /// `report_slave_id` - returns a description of the controller
    ///
    /// The [`report_slave_id()`](#method.report_slave_id) function shall send a request to the controller to obtain a
    /// description of the controller. The response stored in `dest` contains:
    ///     * the slave ID, this unique ID is in reality not unique at all so it's not possible to depend on it to know
    /// how the information are packed in the response.
    ///     * the run indicator status (0x00 = OFF, 0xFF = ON)
    ///     * additional data specific to each controller. For example, libmodbus returns the version of the library as
    /// a string.
    ///
    /// # Return value
    ///
    /// The function returns a `Result` containing the number of read bits if successful. If the output was truncated
    /// due the `max_dest` limit then the return value is the number of bytes which would have been written to `dest`.
    /// Thus, a return value greater than the `max_dest` means that the resonse data was truncated.
    /// Otherwise the Result contains an Error.
    ///
    /// # Parameters
    ///
    /// * `max_dest`    - limit, write `max_dest` bytes from the response to `dest`
    /// * `dest`    - the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut bytes = vec![0u8; Modbus::MAX_PDU_LENGTH];
    ///
    /// assert!(modbus.report_slave_id(Modbus::MAX_PDU_LENGTH, &mut bytes).is_ok());
    /// // assert_eq!(bytes, vec![180, 255, 76, 77, 66, 51, 46, 49, 46, 52]));
    /// ```
    fn report_slave_id(&self, max_dest: usize, dest: &mut [u8]) -> Result<u16> {
        unsafe {
            match ffi::modbus_report_slave_id(self.ctx, Modbus::MAX_PDU_LENGTH as i32, dest.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }

    /// `write_bit` - write a single bit
    ///
    /// The [`write_bit()`](#method.write_bit) function shall write the `status` at the `address` of the remote device.
    /// The value must be set to `true` of `false`.
    ///
    /// The function uses the Modbus function code 0x05 (force single coil).
    ///
    /// # Return value
    ///
    /// The function return an OK Result, containing a one, if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `status` - status that should write at the address `addr`
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    ///
    /// assert!(modbus.write_bit(address, true).is_ok());
    /// ```
    fn write_bit(&self, address: u16, status: bool) -> Result<()> {
        unsafe {
            match ffi::modbus_write_bit(self.ctx, address as c_int, status as c_int) {
                -1 => Err(get_error(Error::last_os_error())),
                1 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `write_register` - write a single register
    ///
    /// The [`write_register()`](#method.write_register) function shall write the value of value holding registers at
    /// the address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x06 (preset single register).
    ///
    /// # Return value
    ///
    /// The function return an OK Result, containing a one, if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `value` - vec with the value of the holding register which shall be written
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let value = u16::max_value();
    ///
    /// assert!(modbus.write_register(address, value).is_ok());
    /// ```
    fn write_register(&self, address: u16, value: u16) -> Result<()> {
        unsafe {
            match ffi::modbus_write_register(self.ctx, address as c_int, value as c_int) {
                -1 => Err(get_error(Error::last_os_error())),
                1 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `write_bits` - write many bits
    ///
    /// The [`write_bits()`](#method.write_bits) function shall write the status of the bits (coils) from `src` at the
    /// `address` of the remote device. The `src` array must contains bytes set to TRUE or FALSE.
    ///
    /// The function shall return the number of written bits if successful. Otherwise it contains an Error.
    ///
    /// # Return value
    ///
    /// The function returns a Ok Result containing the number of written bits. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number or bits that should be writen at the address `address`
    /// * `src`     - vec of `0` and `1` (true and false) values
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let tab_bytes = vec![0u8];
    ///
    /// assert_eq!(modbus.write_bits(address, 1, &tab_bytes).unwrap(), 1);
    /// ```
    fn write_bits(&self, address: u16, num: u16, src: &[u8]) -> Result<u16> {
        unsafe {
            match ffi::modbus_write_bits(self.ctx, address as c_int, num as c_int, src.as_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                num => Ok(num as u16),
            }
        }
    }

    /// `write_registers` - write many registers
    ///
    /// The [`write_registers()`](#method.write_registers) function shall write the content of the `num` holding
    /// registers
    /// from the array `src` at `address` of the remote device.
    ///
    /// The function uses the Modbus function code 0x10 (preset multiple registers).
    ///
    /// # Return value
    ///
    /// The function returns a Ok Result containing the number of written bytes. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num`     - number of holding registers that should write at the address `address`
    /// * `src`     - holding register
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let tab_bytes = vec![0u16];
    ///
    /// assert_eq!(modbus.write_registers(address, 1, &tab_bytes).unwrap(), 1);
    /// ```
    fn write_registers(&self, address: u16, num: u16, src: &[u16]) -> Result<u16> {
        unsafe {
            match ffi::modbus_write_registers(self.ctx, address as c_int, num as c_int, src.as_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                num => Ok(num as u16),
            }
        }
    }

    /// `write_and_read_registers` - write and read many registers in a single transaction
    ///
    /// The [`write_and_read_registers()`](#method.write_and_read_registers) function shall write the content of the
    /// write_nb holding registers from the array src to the address write_addr of the remote device then shall read
    /// the content of the read_nb holding registers to the address read_addr of the remote device. The result of
    /// reading is stored in dest array as word values (16 bits).
    ///
    /// The function uses the Modbus function code 0x17 (write/read registers).
    ///
    /// # Return value
    ///
    /// The function returns a Ok Result containing the number of read registers. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `write_address`   - address of the remote device
    /// * `write_num`       - number of holding registers
    /// * `src`             - holding register
    /// * `read_address`    - address of the remote device
    /// * `read_num`        - number of holding registers
    /// * `dest`            - holding register
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let request_bytes = vec![1u16];
    /// let mut response_bytes = vec![0u16];
    ///
    /// assert_eq!(modbus.write_and_read_registers(
    ///                 address, 1, &request_bytes,
    ///                 address, 1, &mut response_bytes).unwrap(), 1);
    /// ```
    fn write_and_read_registers(&self, write_address: u16, write_num: u16, src: &[u16], read_address: u16,
                                read_num: u16, dest: &mut [u16])
                                -> Result<u16> {
        unsafe {
            match ffi::modbus_write_and_read_registers(self.ctx,
                                                                 write_address as c_int,
                                                                 write_num as c_int,
                                                                 src.as_ptr(),
                                                                 read_address as c_int,
                                                                 read_num as c_int,
                                                                 dest.as_mut_ptr()) {
                                                                     -1 => Err(get_error(Error::last_os_error())),
                num => Ok(num as u16),
            }
        }
    }

    /// `mask_write_register` - mask a single register
    ///
    /// The [`mask_write_register()`](#method.mask_write_register) function shall modify the value of the
    /// holding register at the address `address` of the remote device using the algorithm:
    ///
    /// ```bash,no_run
    /// new value = (current value AND 'and') OR ('or' AND (NOT 'and'))
    /// ```
    ///
    /// The function uses the **Modbus function code 0x16** (mask single register).
    ///
    /// # Return value
    ///
    /// The function returns a Ok Result if succesful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `address`    - address of the remote device
    /// * `and_mask`   - AND mask
    /// * `or_mask`    - OR mask
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.mask_write_register(1, 0xF2, 0x25).is_ok());
    /// ```
    fn mask_write_register(&self, address: u16, and_mask: u16, or_mask: u16) -> Result<()> {
        unsafe {
            match ffi::modbus_mask_write_register(self.ctx, address as c_int, and_mask, or_mask) {
                -1 => Err(get_error(Error::last_os_error())),
                1 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `send_raw_request` - send a raw request
    ///
    /// The [`send_raw_request()`](#method.send_raw_request) function shall send a request via the socket of the
    /// current modbus contest.
    /// This function must be used for debugging purposes because you have to take care to make a valid request by hand.
    /// The function only adds to the message, the header or CRC of the selected backend, so `raw_request` must start
    /// and contain at least a slave/unit identifier and a function code.
    /// This function can be used to send request not handled by the library.
    ///
    /// The enum [`FunctionCode`](enum.FunctionCode.html) provides a list of supported Modbus functions codes, to help
    /// build of raw requests.
    ///
    /// # Parameters
    ///
    /// * `raw_request`     - raw request to send
    /// * `length`          - raw request length
    ///
    /// # Return value
    ///
    /// The function returns a Result, containing the full message lenght,  counting the extra data relating to the
    /// backend, if successful. Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, FunctionCode};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::ReadHoldingRegisters as u8, 0x00, 0x01, 0x0, 0x05];
    /// let mut response = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];
    ///
    /// assert_eq!(modbus.send_raw_request(&mut raw_request, 6 * std::mem::size_of::<u8>() as i32).unwrap(), 12);
    /// assert!(modbus.receive_confirmation(&mut response).is_ok());
    /// ```
    fn send_raw_request(&self, raw_request: &mut [u8], lenght: i32) -> Result<u16> {
        unsafe {
            match ffi::modbus_send_raw_request(self.ctx,
                                                         raw_request.as_mut_ptr(),
                                                         lenght) {
                -1 => Err(get_error(Error::last_os_error())),
                num => Ok(num as u16),
            }
        }
    }

    /// `receive_confirmation` - receive a confirmation request
    ///
    /// The [`receive_confirmation()`](#method.receive_confirmation) function shall receive a request via the socket of
    /// the context `ctx` Member of the [Modbus struct](struct.Modbus.html).
    /// This function must be used for debugging purposes because the received response isn’t checked against the
    /// initial request.
    /// This function can be used to receive request not handled by the library.
    ///
    /// The maximum size of the response depends on the used backend,
    /// in RTU the `response` array must be `Modbus::RTU_MAX_ADU_LENGTH` bytes and in TCP it must be
    /// `Modbus::TCP_MAX_ADU_LENGTH` bytes.
    /// If you want to write code compatible with both, you can use the constant MODBUS_MAX_ADU_LENGTH (maximum value
    /// of all libmodbus backends).
    /// Take care to allocate enough memory to store responses to avoid crashes of your server.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing the response length if successful. The returned request length can be
    /// zero if the indication request is ignored (eg. a query for another slave in RTU mode).
    /// Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `response`   - store for the received response
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut response = vec![0u8; Modbus::MAX_ADU_LENGTH];
    ///
    /// assert!(modbus.receive_confirmation(&mut response).is_ok());
    /// ```
    fn receive_confirmation(&self, response: &mut [u8]) -> Result<u16> {
        unsafe {
            match ffi::modbus_receive_confirmation(self.ctx, response.as_mut_ptr()) {
                -1 => Err(get_error(Error::last_os_error())),
                len => Ok(len as u16),
            }
        }
    }
}
