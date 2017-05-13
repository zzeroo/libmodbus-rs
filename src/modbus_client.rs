use errors::*;
use libc::c_int;
use libmodbus_sys;
use modbus::Modbus;


/// The Modbus protocol defines different data types and functions to read and write them from/to remote devices.
/// The following functions are used by the clients to send Modbus requests:
///
/// * Read data
///     - [`read_bits()`](struct.Modbus.html#method.read_bitsmode), [`read_input_bits()`](struct.Modbus.html#method.read_input_bits), [`read_registers()`](struct.Modbus.html#method.read_registers), [`read_input_registers()`](struct.Modbus.html#method.read_input_registers), [`report_slave_id()`](struct.Modbus.html#method.report_slave_id)
/// * Write data
///     - [`write_bit()`](struct.Modbus.html#method.write_bit), [`write_register()`](struct.Modbus.html#method.write_register), [`write_bits()`](struct.Modbus.html#method.write_bits), [`write_registers()`](struct.Modbus.html#method.write_registers)
/// * Write and read data
///     - [`write_and_read_registers()`](struct.Modbus.html#method.write_and_read_registers)
/// * Raw requests
///     - [`send_raw_request()`](struct.Modbus.html#method.send_raw_request), [`receive_confirmation()`](struct.Modbus.html#method.receive_confirmation)
/// * Reply an exception
///     - [`reply_exception()`](struct.Modbus.html#method.reply_exception)
///
pub trait ModbusClient {
    fn read_bits(&self, address: i32, num_bit: i32, dest: &mut [u8]) -> i32;
    fn read_input_bits(&self, address: i32, num_bit: i32, dest: &mut [u8]) -> i32;
    fn read_registers(&self, address: i32, num_bit: i32, dest: &mut [u16]) -> i32;
    fn read_input_registers(&self, address: i32, num_bit: i32, dest: &mut [u16]) -> i32;
    fn report_slave_id(&self, max_dest: i32, dest: &mut [u8]) -> i32;
    fn write_bit(&self, address: i32, status: bool) -> i32;
    fn write_bits(&self, address: i32, num_bit: i32, src: &[u8]) -> i32;
    fn write_register(&self, address: i32, value: i32) -> i32;
    fn write_registers(&self, address: i32, num_bit: i32, src: &[u16]) -> i32;
    fn write_and_read_registers(&self, write_address: i32, write_num_bit: i32, src: &[u16],
                                       read_address: i32, read_num_bit: i32, dest: &mut [u16]) -> i32;
    fn send_raw_request(&self, raw_request: &mut [u8]) -> i32;
    fn receive_confirmation(&self, response: &mut[u8]) -> i32;
    fn reply_exception(&self, request: &[u8], exception_code: u32) -> i32;
}

// TODO: add real, working examples
impl ModbusClient for Modbus {
    /// `read_bits` - read many bits
    ///
    /// The [`read_bits()`](#method.read_bits) function shall read the status of the nb bits (coils) to the address of the remote device.
    /// The result of reading is stored in `Result<Vec<u8>>`.
    ///
    /// The function uses the Modbus function code 0x01 (read coil status).
    ///
    /// # Return value
    ///
    /// The function shall return the number of read bits if successful. Otherwise it shall return -1 .
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - number of coils to read
    /// * `dest` - If successful the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let mut dest = vec![0u8; 100];
    ///
    /// assert_eq!(modbus.read_bits(address, 1, &mut dest), -1); // -1 because there is no active server
    /// ```
    fn read_bits(&self, address: i32, num_bit: i32, dest: &mut [u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_read_bits(self.ctx, address as c_int, num_bit, dest.as_mut_ptr())
        }
    }

    /// `read_input_bits` - read many input bits
    ///
    /// The [`read_input_bits()`](#method.read_input_bits) function shall read the content of the nb input bits to the address addr of the remote device.
    /// The result of reading is stored in dest array as unsigned bytes (8 bits) set to TRUE or FALSE.
    ///
    /// The function uses the Modbus function code 0x02 (read input status).
    ///
    /// # Return value
    ///
    /// The function shall return the number of read input status if successful. Otherwise it shall return -1 .
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - number of input bits to read
    /// * `dest` - If successful the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let mut dest = vec![0u8];
    ///
    /// assert_eq!(modbus.read_input_bits(address, 1, &mut dest), -1); // -1 because there is no active server
    /// ```
    fn read_input_bits(&self, address: i32, num_bit: i32, dest: &mut [u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_read_input_bits(self.ctx, address as c_int, num_bit, dest.as_mut_ptr())
        }
    }

    /// `read_registers` - read many registers
    ///
    /// The [`read_registers()`](#method.read_registers) function shall read the content of the nb holding registers to the address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x03 (read holding registers).
    ///
    /// # Return value
    ///
    /// The function shall return the number of read registers if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - number of holding registers to read
    /// * `dest` - If successful the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let mut dest = vec![0u16; 100];
    ///
    /// assert_eq!(modbus.read_registers(address, 1, &mut dest), -1); // -1 because there is no active server
    /// ```
    fn read_registers(&self, address: i32, num_bit: i32, dest: &mut [u16]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_read_registers(self.ctx, address as c_int, num_bit, dest.as_mut_ptr())
        }
    }

    /// `read_input_registers` -  read many input registers
    ///
    /// The [`read_input_registers()`](#method.read_input_registers) function shall read the content of the nb input registers to address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x04 (read input registers). The holding registers and input registers have different historical meaning,
    /// but nowadays it’s more common to use holding registers only.
    ///
    /// # Return value
    ///
    /// The function shall return the number of read input registers if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - number of input registers to read
    /// * `dest` - If successful the result of the reading is stored here
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let mut dest = vec![0u16; 100];
    ///
    /// assert_eq!(modbus.read_registers(address, 1, &mut dest), -1); // -1 because there is no active server
    /// ```
    fn read_input_registers(&self, address: i32, num_bit: i32, dest: &mut [u16]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_read_input_registers(self.ctx, address as c_int, num_bit, dest.as_mut_ptr())
        }
    }

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
    /// # Return value
    ///
    /// The function shall return the number of read data if successful.
    ///
    /// If the output was truncated due to the `max_dest` limit then the return value is the number of bytes which would have been written
    /// to `dest` if enough space had been available. Thus, a return value greater than `max_dest` means that the response data was truncated.
    ///
    /// Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `max_dest` - limit the return value to max_dest bytes
    /// * `dest` - the response is stored here
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, MODBUS_MAX_PDU_LENGTH};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut tab_bytes = vec![0u8; MODBUS_MAX_PDU_LENGTH as usize];
    ///
    /// let rc = modbus.report_slave_id(MODBUS_MAX_PDU_LENGTH as i32, &mut tab_bytes);
    /// if (rc > 1) {
    ///     println!("Run Status Indicator: {}", match tab_bytes[1] { 1 => "ON", _ => "OFF"} );
    /// }
    /// ```
    fn report_slave_id(&self, max_dest: i32, dest: &mut [u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_report_slave_id(self.ctx, max_dest, dest.as_mut_ptr())
        }
    }

    /// `write_bit` - read many registers
    ///
    /// The [`write_bit()`](#method.write_bit) function shall write the status of status at the address addr of the remote device.
    /// The value must be set to `true` of `false`.
    ///
    /// The function uses the Modbus function code 0x05 (force single coil).
    ///
    /// # Return value
    ///
    /// The function shall return 1 if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `status` - status that should write at the address `addr`
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    ///
    /// assert_eq!(modbus.write_bit(address, true), -1); // -1 because there is no active server
    /// ```
    fn write_bit(&self, address: i32, status: bool) -> i32 {
        unsafe {
            libmodbus_sys::modbus_write_bit(self.ctx, address as c_int, status as c_int)
        }
    }

    /// `write_register` - write a single register
    ///
    /// The [`write_register()`](#method.write_register) function shall write the value of value holding registers at the address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x06 (preset single register).
    ///
    /// # Return value
    ///
    /// The function shall return 1 if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `value` - vec with the value of the holding register which shall be written
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let value = i32::max_value();
    ///
    /// assert_eq!(modbus.write_register(address, value), -1); // -1 because there is no active server
    /// ```
    fn write_register(&self, address: i32, value: i32) -> i32 {
        unsafe {
            libmodbus_sys::modbus_write_register(self.ctx, address as c_int, value as c_int)
        }
    }

    /// `write_bits` - write many bits
    ///
    /// The [`write_bits()`](#method.write_bits) function shall write the status of the nb bits (coils) from src at the address addr of the remote device. The src array must contains bytes set to TRUE or FALSE.
    ///
    /// The function shall return the number of written bits if successful. Otherwise it shall return -1.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing a `1` if successful, or an Error.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - status that should write at the address `addr`
    /// * `src` - vec of `0` and `1` (true and false) values
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let tab_bytes = vec![0u8];
    ///
    /// assert_eq!(modbus.write_bits(address, 1, &tab_bytes), -1); // -1 because there is no active server
    /// ```
    fn write_bits(&self, address: i32, num_bit: i32, src: &[u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_write_bits(self.ctx, address as c_int, num_bit, src.as_ptr())
        }
    }

    /// `write_registers` - write many registers
    ///
    /// The [`write_registers()`](#method.write_registers) function shall write the content of the nb holding registers from the array src at address addr of the remote device.
    ///
    /// The function uses the Modbus function code 0x10 (preset multiple registers).
    ///
    /// # Return value
    ///
    /// The function shall return the number of written registers if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `address` - address of the remote device
    /// * `num_bit` - number of holding registers that should write at the address `addr`
    /// * `src` - holding register
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let address = 1;
    /// let tab_bytes = vec![0u16];
    ///
    /// assert_eq!(modbus.write_registers(address, 1, &tab_bytes), -1); // -1 because there is no active server
    /// ```
    fn write_registers(&self, address: i32, num_bit: i32, src: &[u16]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_write_registers(self.ctx, address as c_int, num_bit, src.as_ptr())
        }
    }

    /// `write_and_read_registers` - write and read many registers in a single transaction
    ///
    /// The [`write_and_read_registers()`](#method.write_and_read_registers) function shall write the content of the write_nb holding registers from the array src to the address write_addr of the remote device then shall read the content of the read_nb holding registers to the address read_addr of the remote device. The result of reading is stored in dest array as word values (16 bits).
    ///
    /// The function uses the Modbus function code 0x17 (write/read registers).
    ///
    /// # Return value
    ///
    /// The function shall return the number of read registers if successful. Otherwise it shall return -1.
    ///
    /// # Parameters
    ///
    /// * `write_address`   - address of the remote device
    /// * `write_num_bit`   - number of holding registers
    /// * `src`             - holding register
    /// * `read_address`    - address of the remote device
    /// * `read_num_bit`    - number of holding registers
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
    ///                 address, 1, &mut response_bytes), 1);
    /// ```
    fn write_and_read_registers(&self, write_address: i32, write_num_bit: i32, src: &[u16],
                                       read_address: i32, read_num_bit: i32, dest: &mut [u16]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_write_and_read_registers(self.ctx, write_address as c_int, write_num_bit, src.as_ptr(),
                                                                    read_address as c_int, read_num_bit, dest.as_mut_ptr())
        }
    }

    /// `send_raw_request` - send a raw request
    ///
    /// The [`send_raw_request()`](#method.send_raw_request) function shall send a request via the socket of the current modbus contest.
    /// This function must be used for debugging purposes because you have to take care to make a valid request by hand.
    /// The function only adds to the message, the header or CRC of the selected backend, so `raw_req` must start and contain at least a slave/unit identifier and a function code.
    /// This function can be used to send request not handled by the library.
    ///
    /// The public header of libmodbus provides a list of supported Modbus functions codes,
    /// prefixed by MODBUS_FC_ (eg. MODBUS_FC_READ_HOLDING_REGISTERS), to help build of raw requests.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing the full message length, counting the extra data relating to the backend, if successful, or an Error.
    ///
    /// # Parameters
    ///
    /// * `raw_request`   - raw request to send
    ///
    /// # Return value
    ///
    /// The function returns a Result, containing the full message lenght,  counting the extra data relating to the backend, if successful. Or an Error.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, FunctionCode, MODBUS_TCP_MAX_ADU_LENGTH};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::READ_HOLDING_REGISTERS as u8, 0x00, 0x01, 0x0, 0x05];
    /// let mut response: Vec<u8> = vec![0; MODBUS_TCP_MAX_ADU_LENGTH as usize];
    ///
    /// assert_eq!(modbus.send_raw_request(&mut raw_request), 12);
    /// assert_eq!(modbus.receive_confirmation(&mut response), 19);
    /// ```
    fn send_raw_request(&self, raw_request: &mut [u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_send_raw_request(self.ctx, raw_request.as_mut_ptr(), raw_request.len() as c_int)
        }
    }

    /// `receive_confirmation` - receive a confirmation request
    ///
    /// The [`receive_confirmation()`](#method.receive_confirmation) function shall receive a request via the socket of the context ctx Member of the [Modbus struct](struct.Modbus.html).
    /// This function must be used for debugging purposes because the received response isn’t checked against the initial request.
    /// This function can be used to receive request not handled by the library.
    ///
    /// The maximum size of the response depends on the used backend,
    /// in RTU the rsp array must be MODBUS_RTU_MAX_ADU_LENGTH bytes and in TCP it must be MODBUS_TCP_MAX_ADU_LENGTH bytes.
    /// If you want to write code compatible with both, you can use the constant MODBUS_MAX_ADU_LENGTH (maximum value of all libmodbus backends).
    /// Take care to allocate enough memory to store responses to avoid crashes of your server.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing the response length if successful, or an Error.
    /// The returned request length can be zero if the indication request is ignored (eg. a query for another slave in RTU mode).
    ///
    /// # Parameters
    ///
    /// * `response`   - store for the received response
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, MODBUS_MAX_ADU_LENGTH};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut response: Vec<u8> = vec![0; MODBUS_MAX_ADU_LENGTH as usize];
    ///
    /// modbus.receive_confirmation(&mut response);
    /// ```
    fn receive_confirmation(&self, response: &mut[u8]) -> i32 {
        unsafe {
            libmodbus_sys::modbus_receive_confirmation(self.ctx, response.as_mut_ptr())
        }
    }

    /// TODO: create an enum for the exception codes
    ///
    /// `reply_exception` - send an exception reponse
    ///
    /// The modbus_reply_exception() function shall send an exception response based on the exception_code in argument.
    ///
    /// The libmodbus provides the following exception codes:
    ///
    /// * MODBUS_EXCEPTION_ILLEGAL_FUNCTION (1)
    /// * MODBUS_EXCEPTION_ILLEGAL_DATA_ADDRESS (2)
    /// * MODBUS_EXCEPTION_ILLEGAL_DATA_VALUE (3)
    /// * MODBUS_EXCEPTION_SLAVE_OR_SERVER_FAILURE (4)
    /// * MODBUS_EXCEPTION_ACKNOWLEDGE (5)
    /// * MODBUS_EXCEPTION_SLAVE_OR_SERVER_BUSY (6)
    /// * MODBUS_EXCEPTION_NEGATIVE_ACKNOWLEDGE (7)
    /// * MODBUS_EXCEPTION_MEMORY_PARITY (8)
    /// * MODBUS_EXCEPTION_NOT_DEFINED (9)
    /// * MODBUS_EXCEPTION_GATEWAY_PATH (10)
    /// * MODBUS_EXCEPTION_GATEWAY_TARGET (11)
    ///
    /// The initial request req is required to build a valid response.
    ///
    /// # Return value
    ///
    /// The function returns the length of the response sent if successful, or an Error.
    ///
    /// # Parameters
    ///
    /// * `request`         - initial request, required to build a valid response
    /// * `exception_code`  - Exception Code
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// use libmodbus_rs::Exception;
    ///
    /// let request: Vec<u8> = vec![0x01];
    /// assert_eq!(modbus.reply_exception(&request, Exception::ACKNOWLEDGE as u32), 9);
    /// ```
    fn reply_exception(&self, request: &[u8], exception_code: u32) -> i32 {
        unsafe {
            libmodbus_sys::modbus_reply_exception(self.ctx, request.as_ptr(), exception_code)
        }
    }
}
