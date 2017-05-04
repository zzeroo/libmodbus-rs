//! libmodbus is a library to send/receive data with a device which respects the Modbus protocol.
//! This library contains various backends to communicate over different networks (eg. serial in RTU mode or Ethernet in TCP/IPv6).
//! The http://www.modbus.org site provides documentation about the protocol at http://www.modbus.org/specs.php.
//!
//! libmodbus provides an abstraction of the lower communication layers and offers the same API on all supported platforms.
//!
//! This documentation presents an overview of libmodbus concepts, describes how libmodbus abstracts Modbus communication with
//! different hardware and platforms and provides a reference manual for the functions provided by the libmodbus library.
//!
//! ## Contexts
//!
//! The Modbus protocol contains many variants (eg. serial RTU or Ehternet TCP), to ease the implementation of a variant,
//! the library was designed to use a backend for each variant.
//! The backends are also a convenient way to fulfill other requirements (eg. real-time operations). Each backend offers a specific function to create a new modbus_t context.
//! The modbus_t context is an opaque structure containing all necessary information to establish a connection with others Modbus devices according to the selected variant.
//!
//! You can choose the best context for your needs among:
//!
//! * [RTU Context](modbus_rtu)
//! * [TCP (IPv4) Context]
//! * [TCP PI (IPv4 and IPv6) Context]
//!
use libmodbus_sys;
use std::io::{Error, ErrorKind};
use libc::c_int;


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
    pub fn set_byte_timeout(&mut self, to_sec: u32, to_usec: u32) -> Result<(i32, i32), Error> {
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
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), (0, 0));
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
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), (0, 0));
    /// ```
    pub fn set_response_timeout(&mut self, to_sec: u32, to_usec: u32) -> Result<(i32, i32), Error> {
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
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), (0, 0));
    /// ```
    pub fn set_error_recovery(&mut self, modbus_error_recovery_mode: libmodbus_sys::modbus_error_recovery_mode) -> Result<i32, Error> {
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


impl Drop for Modbus {
    fn drop(&mut self) {
        self.close();
        self.free();
    }
}
