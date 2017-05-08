// use errors::*;
use libmodbus_sys;
use std::io::{Error, ErrorKind};
use libc::{c_int, c_uint};


/// Safe interface for [libmodbus](http://libmodbus.org)
///
/// The different parts of libmodbus are implemented as traits. The modules of this crate contains these
/// traits and a implementation with a, hopefully safe, interface.
///
pub struct Modbus {
    pub ctx: *mut libmodbus_sys::modbus_t,
}

impl Modbus {
    /// `connect` - establish a Modbus connection
    ///
    /// The [`connect()`](#method.connect) function shall establish a connection to a Modbus server,
    /// a network or a bus.
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
    pub fn connect(&self) -> Result<i32, Error> {
        unsafe {
            match libmodbus_sys::modbus_connect(self.ctx) {
                -1 => Err(Error::last_os_error()),
                _ => Ok(0),
            }
        }
    }

    /// `flush` - flush non-transmitted data
    /// The [`flush()`](#method.flush) function shall discard data received but not read to the socket or file descriptor associated to the context ctx.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.flush().is_ok());
    /// ```
    pub fn flush(&self) -> Result<i32, Error> {
        unsafe {
            match libmodbus_sys::modbus_flush(self.ctx) {
                -1 => Err(Error::last_os_error()),
                _ => Ok(0),
            }
        }
    }

    /// `set_slave` - set slave number in the context
    ///
    /// The [`set_slave()`](#method.set_slave) function shall set the slave number in the libmodbus context.
    /// The behavior depends of network and the role of the device:
    ///
    /// RTU
    ///     Define the slave ID of the remote device to talk in master mode or set the internal slave ID in slave mode.
    ///     According to the protocol, a Modbus device must only accept message holding its slave number or the special broadcast number.
    /// TCP
    ///     The slave number is only required in TCP if the message must reach a device on a serial network.
    ///     Some not compliant devices or software (such as modpoll) uses the slave ID as unit identifier,
    ///     that’s incorrect (cf page 23 of Modbus Messaging Implementation Guide v1.0b) but without the slave value,
    ///     the faulty remote device or software drops the requests!
    ///     The special value MODBUS_TCP_SLAVE (0xFF) can be used in TCP mode to restore the default value.
    ///     The broadcast address is MODBUS_BROADCAST_ADDRESS.
    ///     This special value must be use when you want all Modbus devices of the network receive the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusRTU};
    ///
    /// const YOUR_DEVICE_ID: i32 = 1;
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    /// modbus.set_slave(YOUR_DEVICE_ID);
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {}
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn set_slave(&mut self, slave: i32) -> Result<i32, Error> {
        unsafe {
            match libmodbus_sys::modbus_set_slave(self.ctx, slave as c_int) {
                -1 => Err(Error::new(ErrorKind::Other, "The slave number is invalid.")),
                _ => Ok(0),
            }
        }
    }

    /// `set_debug` - set debug flag of the context
    ///
    /// The [`set_debug()`](#method.set_debug) function shall set the debug flag of the modbus_t context by using the argument flag.
    /// By default, the boolean flag is set to FALSE. When the flag value is set to TRUE, many verbose messages are displayed on stdout and stderr.
    /// For example, this flag is useful to display the bytes of the Modbus messages.
    ///
    /// ```bash
    /// [00][14][00][00][00][06][12][03][00][6B][00][03]
    /// Waiting for a confirmation…
    /// <00><14><00><00><00><09><12><03><06><02><2B><00><00><00><00>
    /// ```
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// modbus.set_debug(true);
    /// ```
    pub fn set_debug(&mut self, flag: bool) -> Result<i32, Error> {
        unsafe {
            match libmodbus_sys::modbus_set_debug(self.ctx, flag as c_int) {
                -1 => Err(Error::new(ErrorKind::Other, "Invalid flag")),
                _ => Ok(0),
            }
        }
    }

    /// `get_byte_timeout` - get timeout between bytes
    ///
    /// [`get_byte_timeout()`](#method.get_byte_timeout) function returns a tupple with the timeout interval between two consecutive bytes of the same message `Result<(to_sec, to_usec), Error>`.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), (0, 0));
    /// ```
    pub fn get_byte_timeout(&self) -> Result<(i32, i32), Error> {
        unimplemented!()
    }

    /// `set_byte_timeout` - set timeout between bytes
    ///
    /// The [`set_byte_timeout()`](#method.set_byte_timeout) function shall set the timeout interval between two consecutive bytes of the same message.
    /// The timeout is an upper bound on the amount of time elapsed before select() returns, if the time elapsed is longer than the defined timeout,
    /// an ETIMEDOUT error will be raised by the function waiting for a response.
    ///
    /// The value of **to_usec** argument must be in the range 0 to 999999.
    ///
    /// If both **to_sec** and **to_usec** are zero, this timeout will not be used at all. In this case, [`set_byte_timeout()`](#method.set_byte_timeout)
    /// governs the entire handling of the response, the full confirmation response must be received before expiration of the response timeout.
    /// When a byte timeout is set, the response timeout is only used to wait for until the first byte of the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), (0, 0));
    /// ```
    pub fn set_byte_timeout(&mut self, _to_sec: u32, _to_usec: u32) -> Result<(i32, i32), Error> {
        unimplemented!()
    }

    /// `get_response_timeout` - get timeout for response
    ///
    /// The [`get_response_timeout()`](#method.get_response_timeout) function shall return the timeout interval used to wait for a response
    /// in the **to_sec** and **to_usec** arguments.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn get_response_timeout(&self) -> Result<(i32, i32), Error> {
        unimplemented!()
    }

    /// `set_response_timeout` - set timeout for response
    ///
    /// The [`set_response_timeout()`](#method.set_response_timeout) function shall set the timeout interval used to wait for a response.
    /// When a byte timeout is set, if elapsed time for the first byte of response is longer than the given timeout,
    /// an ETIMEDOUT error will be raised by the function waiting for a response. When byte timeout is disabled,
    /// the full confirmation response must be received before expiration of the response timeout.
    ///
    /// The value of **to_usec** argument must be in the range 0 to 999999.
    ///
    /// If both **to_sec** and **to_usec** are zero, this timeout will not be used at all. In this case, [`set_response_timeout()`](#method.set_response_timeout)
    /// governs the entire handling of the response, the full confirmation response must be received before expiration of the response timeout.
    /// When a byte timeout is set, the response timeout is only used to wait for until the first byte of the response.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn set_response_timeout(&mut self, _to_sec: u32, _to_usec: u32) -> Result<(i32, i32), Error> {
        unimplemented!()
    }

    /// `set_error_recovery` - set the error recovery mode
    ///
    /// The [`set_error_recovery()`](#method.set_error_recovery) function shall set the error recovery mode to apply when the connection fails or
    /// the byte received is not expected. The argument error_recovery may be bitwise-or’ed with zero or more of the following constants.
    ///
    /// By default there is no error recovery (MODBUS_ERROR_RECOVERY_NONE) so the application is responsible for controlling the error values
    /// returned by libmodbus functions and for handling them if necessary.
    ///
    /// When MODBUS_ERROR_RECOVERY_LINK is set, the library will attempt an reconnection after a delay defined by response timeout of the libmodbus context.
    /// This mode will try an infinite close/connect loop until success on send call and will just try one time to re-establish the connection on
    /// select/read calls (if the connection was down, the values to read are certainly not available any more after reconnection, except for slave/server).
    /// This mode will also run flush requests after a delay based on  the current response timeout in some situations (eg. timeout of select call).
    /// The reconnection attempt can hang for several seconds if the network to the remote target unit is down.
    ///
    /// When MODBUS_ERROR_RECOVERY_PROTOCOL is set, a sleep and flush sequence will be used to clean up the ongoing communication, this can
    /// occurs when the message length is invalid, the TID is wrong or the received function code is not the expected one.
    /// The response timeout delay will be used to sleep.
    ///
    /// The modes are mask values and so they are complementary.
    ///
    /// It’s not recommended to enable error recovery for slave/server.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn set_error_recovery(&mut self, _modbus_error_recovery_mode: libmodbus_sys::modbus_error_recovery_mode) -> Result<i32, Error> {
        unimplemented!()
    }

    /// `set_socket` - set socket of the context
    ///
    /// The [`set_socket()`](#method.set_socket) function shall set the socket or file descriptor in the libmodbus context.
    /// This function is useful for managing multiple client connections to the same server.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn set_socket(&mut self, _socket: u32) -> Result<i32, Error> {
        unimplemented!()
    }

    /// `get_socket` - set socket of the context
    ///
    /// The [`get_socket()`](#method.get_socket) function shall return the current socket or file descriptor of the libmodbus context.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn get_socket(&self) -> Result<u32, Error> {
        unimplemented!()
    }

    /// `get_header_length` - retrieve the current header length
    ///
    /// The [`get_header_length()`](#method.get_header_length) function shall retrieve the current header length from the backend.
    /// This function is convenient to manipulate a message and so its limited to low-level operations.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// ```
    pub fn get_header_length(&self) -> Result<u32, Error> {
        unimplemented!()
    }

    /// `close` - close a Modbus connection
    ///
    /// The [`close()`](#method.close) function shall close the connection established with the backend set in the context.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    ///
    /// modbus.close();
    /// modbus.free();
    /// ```
    pub fn close(&self) {
        unsafe {
            libmodbus_sys::modbus_close(self.ctx);
        }
    }

    /// `free` - free a libmodbus context
    ///
    /// The [`free()`](#method.free) function shall free an allocated modbus_t structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    ///
    /// modbus.close();
    /// modbus.free();
    /// ```
    pub fn free(&mut self) {
        unsafe {
            libmodbus_sys::modbus_free(self.ctx);
        }
    }
}


/// `set_bits_from_byte` - set many bits from a single byte value
///
/// The [`set_bits_from_byte()`](#method.set_bits_from_byte) function shall set many bits from a single byte.
/// All 8 bits from the byte value will be written to dest array starting at index position.
///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_bits_from_byte(_dest: u8, index: c_int, value: u8) {
    unimplemented!()
}

/// `set_bits_from_bytes` -  set many bits from an array of bytes
///
/// The [`set_bits_from_bytes()`](#method.set_bits_from_bytes) function shall set many bits from a single byte.
/// All 8 bits from the byte value will be written to dest array starting at index position.
///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_bits_from_bytes(_dest: u8, _index: c_uint, _num_bits: c_uint, _tab_bytes: Vec<u8>) {
    unimplemented!()
}

/// `get_byte_from_bits` - get the value from many bit
///
/// The [`get_byte_from_bits()`](#method.get_byte_from_bits) function shall extract a value from many bits.
/// All nb_bits bits from src at position index will be read as a single value. To obtain a full byte, set nb_bits to 8.
///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn get_byte_from_bits(_src: u8, index: c_int, _num_bits: c_uint) {
    unimplemented!()
}

/// `get_float_abcd` - get a float value from 2 registers in ABCD byte order
///
/// The [`get_float_abcd()`](#method.get_float_abcd) function shall get a float from 4 bytes in usual Modbus format.
/// The src array must be a pointer on two 16 bits values, for example, if the first word is set to 0x0020 and the second to 0xF147,
/// the float value will be read as 123456.0.
///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn get_float_abcd(_src: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_float_abcd(_dest: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn get_float_badc(_src: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_float_badc(_dest: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn get_float_cdab(_src: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_float_cdab(_dest: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn get_float_dcba(_src: u16) {
    unimplemented!()
}

///
/// # Examples
///
/// ```rust,no_run
/// use modbus_rs::{Modbus, ModbusTCP};
///
/// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// ```
pub fn set_float_dcba(_dest: u16) {
    unimplemented!()
}


impl Drop for Modbus {
    fn drop(&mut self) {
        self.close();
        self.free();
    }
}
