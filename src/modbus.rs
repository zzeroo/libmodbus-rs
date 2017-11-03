#![allow(dead_code)]
use errors::*;
use libc::{c_int, c_uint};
use libmodbus_sys as ffi;
use std::io::Error;

/// Modbus protocol exceptions
///
/// Documentation source: https://en.wikipedia.org/wiki/Modbus#Main_Modbus_exception_codes
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Exception {
    /// (1) Illegal Function - Function code received in the query is not recognized or allowed by slave
    IllegalFunction = 1,
    /// (2) Illegal Data Address - Data address of some or all the required entities are not allowed or do not exist in
    /// slave
    IllegalDataAddress = 2,
    /// (3) Illegal Data Value - Value is not accepted by slave
    IllegalDataValue = 3,
    /// (4) Slave Device Failure - Unrecoverable error occurred while slave was attempting to perform requested action
    SlaveOrServerFailure = 4,
    /// (5) Acknowledge - Slave has accepted request and is processing it, but a long duration of time is required.
    /// This response is returned to prevent a timeout error from occurring in the master. Master can next issue a Poll
    /// Program Complete message to determine whether processing is completed
    Acknowledge = 5,
    /// (6) Slave Device Busy - Slave is engaged in processing a long-duration command. Master should retry later
    SlaveDeviceBusy = 6,
    /// (7) Negative Acknowledge - Slave cannot perform the programming functions. Master should request diagnostic or
    /// error information from slave
    NegativeAcknowledge = 7,
    /// (8) Memory Parity Error - Slave detected a parity error in memory. Master can retry the request, but service
    /// may be required on the slave device
    MemoryParity = 8,
    /// (9) Not defined
    NotDefined = 9,
    /// (10) Gateway Path Unavailable - Specialized for Modbus gateways. Indicates a misconfigured gateway
    GatewayPath = 10,
    /// (11) Gateway Target Device Failed to Respond - Specialized for Modbus gateways. Sent when slave fails to respond
    GatewayTarget = 11,
}

/// Modbus function codes
///
/// Documentation source: https://en.wikipedia.org/wiki/Modbus#Supported_function_codes
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctionCode {
    /// 0x01 Read Coils
    ReadCoils = 1,
    /// 0x02 Read Discrete Inputs
    ReadDiscreteInputs = 2,
    /// 0x03 Read Multiple Holding Registers
    ReadHoldingRegisters = 3,
    /// 0x04 Read Input Registers
    ReadInputRegisters = 4,
    /// 0x05 Write Single Coil
    WriteSingleCoil = 5,
    /// 0x06 Write Single Holding Register
    WriteSingleRegister = 6,
    /// 0x07 Read Exception Status
    ReadExceptionStatus = 7,
    /// 0x15 Write Multiple Coils
    WriteMultipleCoils = 15,
    /// 0x16 Write Multiple Holding Registers
    WriteMultipleRegisters = 16,
    /// 0x17 Report Slave ID
    ReportSlaveId = 17,
    /// 0x22 Mask Write Register
    MaskWriteRegister = 22,
    /// 0x23 Read/Write Multiple Registers
    WriteAndReadRegisters = 23,
}

#[derive(Debug, Copy, Clone)]
pub enum ErrorRecoveryMode {
    Link,
    Protocol,
}

impl ErrorRecoveryMode {
    fn as_raw(&self) -> ffi::modbus_error_recovery_mode {
        use ErrorRecoveryMode::*;

        match *self {
            Link => ffi::modbus_error_recovery_mode_MODBUS_ERROR_RECOVERY_LINK,
            Protocol => ffi::modbus_error_recovery_mode_MODBUS_ERROR_RECOVERY_PROTOCOL,
        }
    }
}

/// Timeout struct
///
/// * The value of **usec** argument must be in the range 0 to 999999.
// For use with timeout methods such as get_byte_timeout and set_byte_timeout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Timeout {
    pub sec: u32,
    pub usec: u32,
}

/// Safe interface for [libmodbus](http://libmodbus.org)
///
/// The different parts of libmodbus are implemented as traits. The modules of this crate contains these
/// traits and a implementation with a, hopefully safe, interface.
///
pub struct Modbus {
    pub ctx: *mut ffi::modbus_t,
}

impl Modbus {
    // Constants
    pub const ENOBASE: u32 = ffi::MODBUS_ENOBASE;
    pub const MAX_ADU_LENGTH: usize = ffi::MODBUS_MAX_ADU_LENGTH as usize;
    pub const MAX_PDU_LENGTH: usize = ffi::MODBUS_MAX_PDU_LENGTH as usize;
    pub const MAX_READ_BITS: usize = ffi::MODBUS_MAX_READ_BITS as usize;
    pub const MAX_READ_REGISTERS: usize = ffi::MODBUS_MAX_READ_REGISTERS as usize;
    pub const MAX_WR_READ_REGISTERS: usize = ffi::MODBUS_MAX_WR_READ_REGISTERS as usize;
    pub const MAX_WR_WRITE_REGISTERS: usize = ffi::MODBUS_MAX_WR_WRITE_REGISTERS as usize;
    pub const MAX_WRITE_BITS: usize = ffi::MODBUS_MAX_WRITE_BITS as usize;
    pub const MAX_WRITE_REGISTERS: usize = ffi::MODBUS_MAX_WRITE_REGISTERS as usize;
    pub const RTU_MAX_ADU_LENGTH: usize = ffi::MODBUS_RTU_MAX_ADU_LENGTH as usize;
    pub const TCP_DEFAULT_PORT: u32 = ffi::MODBUS_TCP_DEFAULT_PORT;
    pub const TCP_MAX_ADU_LENGTH: usize = ffi::MODBUS_TCP_MAX_ADU_LENGTH as usize;
    pub const TCP_SLAVE: u32 = ffi::MODBUS_TCP_SLAVE;

    /// `connect` - establish a Modbus connection
    ///
    /// The [`connect()`](#method.connect) function shall establish a connection to a Modbus server,
    /// a network or a bus.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// // create server
    /// let mut server = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// // create client
    /// let client = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// // start server in listen mode
    /// let _ = server.tcp_listen(1).unwrap();
    ///
    /// assert!(client.connect().is_ok())
    /// ```
    pub fn connect(&self) -> Result<()> {
        unsafe {
            match ffi::modbus_connect(self.ctx) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `flush` - flush non-transmitted data
    ///
    /// The [`flush()`](#method.flush) function shall discard data received but not read to the socket or file
    /// descriptor associated to the context ctx.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.flush().is_ok());
    /// ```
    pub fn flush(&self) -> Result<()> {
        unsafe {
            match ffi::modbus_flush(self.ctx) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `set_slave` - set slave number in the context
    ///
    /// The [`set_slave()`](#method.set_slave) function shall set the slave number in the libmodbus context.
    /// The behavior depends of network and the role of the device:
    ///
    /// * RTU
    ///     - Define the slave ID of the remote device to talk in master mode or set the internal slave ID in slave mode.
    /// According to the protocol, a Modbus device must only accept message holding its slave number or the special
    /// broadcast number.
    /// * TCP
    ///     - The slave number is only required in TCP if the message must reach a device on a serial network.
    ///     Some not compliant devices or software (such as modpoll) uses the slave ID as unit identifier,
    ///     that’s incorrect (see page 23 of Modbus Messaging Implementation Guide v1.0b) but without the slave value,
    ///     the faulty remote device or software drops the requests!
    ///     The special value MODBUS_TCP_SLAVE (0xFF) can be used in TCP mode to restore the default value.
    ///     The broadcast address is MODBUS_BROADCAST_ADDRESS.
    ///     This special value must be use when you want all Modbus devices of the network receive the request.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `slave`   - new slave ID
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusRTU};
    ///
    /// const YOUR_DEVICE_ID: u8 = 1;
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// assert!(modbus.set_slave(YOUR_DEVICE_ID).is_ok());
    /// ```
    pub fn set_slave(&mut self, slave: u8) -> Result<()> {
        unsafe {
            match ffi::modbus_set_slave(self.ctx, slave as c_int) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `get_slave` - get slave number from current context
    ///
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusRTU};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    /// modbus.set_slave(10);
    ///
    /// assert_eq!(modbus.get_slave().unwrap(), 10);
    /// ```
    pub fn get_slave(&self) -> Result<i32> {
        unsafe {
            match ffi::modbus_get_slave(self.ctx) {
                -1 => bail!(Error::last_os_error()),
                num => Ok(num),
            }
        }
    }

    /// `set_debug` - set debug flag of the context
    ///
    /// The [`set_debug()`](#method.set_debug) function shall set the debug flag of the modbus_t context by using the
    /// argument flag.
    /// By default, the boolean flag is set to FALSE. When the flag value is set to TRUE, many verbose messages are
    /// displayed on stdout and stderr.
    /// For example, this flag is useful to display the bytes of the Modbus messages.
    ///
    /// ```bash
    /// [00][14][00][00][00][06][12][03][00][6B][00][03]
    /// Waiting for a confirmation…
    /// <00><14><00><00><00><09><12><03><06><02><2B><00><00><00><00>
    /// ```
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `flag`    - `true` of `false`, enables or disables debug mode
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.set_debug(true).is_ok());
    /// ```
    pub fn set_debug(&mut self, flag: bool) -> Result<()> {
        unsafe {
            match ffi::modbus_set_debug(self.ctx, flag as c_int) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `get_byte_timeout` - get timeout between bytes
    ///
    /// [`get_byte_timeout()`](#method.get_byte_timeout) function returns a
    /// [`Timeout`](struct.Timeout.html) with the timeout interval between
    /// two consecutive bytes of the same message.
    ///
    /// # Return value
    ///
    /// The function return a Result containing a [`Timeout`](struct.Timeout.html) if successful.
    /// Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP, Timeout};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert_eq!(modbus.get_byte_timeout().unwrap(), Timeout { sec: 0, usec: 500000 });
    /// ```
    pub fn get_byte_timeout(&self) -> Result<Timeout> {
        let mut timeout = Timeout { sec: 0, usec: 0 };
        unsafe {
            match ffi::modbus_get_byte_timeout(self.ctx, &mut timeout.sec, &mut timeout.usec) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(timeout),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `set_byte_timeout` - set timeout between bytes
    ///
    /// The [`set_byte_timeout()`](#method.set_byte_timeout) function shall set the timeout interval between two
    /// consecutive bytes of the same message.
    /// The timeout is an upper bound on the amount of time elapsed before select() returns, if the time elapsed is
    /// longer than the defined timeout,
    /// an ETIMEDOUT error will be raised by the function waiting for a response.
    ///
    /// The value of **usec** argument must be in the range 0 to 999999.
    ///
    /// If both **sec** and **usec** are zero, this timeout will not be used at all. In this case,
    /// [`set_byte_timeout()`](#method.set_byte_timeout)
    /// governs the entire handling of the response, the full confirmation response must be received before expiration
    /// of the response timeout.
    /// When a byte timeout is set, the response timeout is only used to wait for until the first byte of the response.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * `timeout`  - Timeout struct with `sec` and `usec`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP, Timeout};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let timeout = Timeout { sec: 1, usec: 500000 };
    /// assert!(modbus.set_byte_timeout(timeout).is_ok());
    /// ```
    pub fn set_byte_timeout(&mut self, timeout: Timeout) -> Result<()> {
        unsafe {
            match ffi::modbus_set_byte_timeout(self.ctx, timeout.sec, timeout.usec) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `get_response_timeout` - get timeout for response
    ///
    /// The [`get_response_timeout()`](#method.get_response_timeout) function shall return the timeout interval used to
    /// wait for a response
    /// in the **sec** and **usec** arguments.
    ///
    /// # Return value
    ///
    /// The function return a Result containing a [`Timeout`](struct.Timeout.html) if successful.
    /// Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP, Timeout};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert_eq!(modbus.get_response_timeout().unwrap(), Timeout { sec: 0, usec: 500000 });
    /// ```
    pub fn get_response_timeout(&self) -> Result<Timeout> {
        let mut timeout = Timeout { sec: 0, usec: 0 };
        unsafe {
            match ffi::modbus_get_response_timeout(self.ctx, &mut timeout.sec, &mut timeout.usec) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(timeout),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `set_response_timeout` - set timeout for response
    ///
    /// The [`set_response_timeout()`](#method.set_response_timeout) function shall set the timeout interval used to
    /// wait for a response.
    /// When a byte timeout is set, if elapsed time for the first byte of response is longer than the given timeout,
    /// an ETIMEDOUT error will be raised by the function waiting for a response. When byte timeout is disabled,
    /// the full confirmation response must be received before expiration of the response timeout.
    ///
    ///
    /// If the [`Timeout`](struct.Timeout.html) members are both **sec** and **usec** are zero,
    /// this timeout will not be used at all. In this case, [`set_response_timeout()`](#method.set_response_timeout)
    /// governs the entire handling of the response, the full confirmation response must be received before expiration
    /// of the response timeout.
    /// When a byte timeout is set, the response timeout is only used to wait for until the first byte of the response.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * [`Timeout`](struct.Timeout.html)  - Timeout
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP, Timeout};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let timeout = Timeout { sec: 1, usec: 500000 };
    /// assert!(modbus.set_response_timeout(timeout).is_ok());
    /// ```
    pub fn set_response_timeout(&mut self, timeout: Timeout) -> Result<()> {
        unsafe {
            match ffi::modbus_set_response_timeout(self.ctx, timeout.sec, timeout.usec) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    /// `set_error_recovery` - set the error recovery mode
    ///
    /// The [`set_error_recovery()`](#method.set_error_recovery) function shall set the error recovery mode to apply
    /// when the connection fails or the byte received is not expected.
    ///
    /// By default there is no error recovery so the application is responsible for controlling the error values
    /// returned by libmodbus functions and for handling them if necessary.
    ///
    /// When `ErrorRecoveryMode::Link` is set, the library will attempt an reconnection after a delay defined by
    /// response timeout ([`set_response_timeout()`](#method.set_response_timeout)) of the libmodbus context.
    /// This mode will try an infinite close/connect loop until success on send call and will just try one time to
    /// re-establish the connection on select/read calls (if the connection was down, the values to read are certainly
    /// not available any more after reconnection, except for slave/server).
    /// This mode will also run flush requests after a delay based on  the current response timeout in some situations
    /// (eg. timeout of select call).
    /// The reconnection attempt can hang for several seconds if the network to the remote target unit is down.
    ///
    /// When `ErrorRecoveryMode::Protocol` is set, a sleep and flush sequence will be used to clean up the ongoing
    /// communication, this can occurs when the message length is invalid, the TID is wrong or the received function
    /// code is not the expected one.
    /// The response timeout delay will be used to sleep.
    ///
    /// The modes are mask values and so they are complementary.
    ///
    /// It’s not recommended to enable error recovery for slave/server.
    ///
    /// # Return value
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Parameters
    ///
    /// * [`ErrorRecoveryMode`](struct.ErrorRecoveryMode.html)  - Timeout
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusTCP, ErrorRecoveryMode};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.set_error_recovery(Some(&[ErrorRecoveryMode::Link, ErrorRecoveryMode::Protocol])).is_ok());
    /// ```
    pub fn set_error_recovery(&mut self, flags: Option<&[ErrorRecoveryMode]>) -> Result<()> {
        let flags = flags.unwrap_or(&[])
            .iter()
            .fold(ffi::modbus_error_recovery_mode_MODBUS_ERROR_RECOVERY_NONE, |acc, v| acc | v.as_raw());

        unsafe {
            match ffi::modbus_set_error_recovery(self.ctx, flags) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => panic!("libmodbus API incompatible response"),
            }
        }
    }

    // TODO: Add examples from: http://zzeroo.github.io/libmodbus-rs/libmodbus/modbus_set_socket.html
    /// `set_socket` - set socket of the context
    ///
    /// The [`set_socket()`](#method.set_socket) function shall set the socket or file descriptor in the libmodbus
    /// context.
    /// This function is useful for managing multiple client connections to the same server.
    ///
    /// # Return values
    ///
    /// The function return an OK Result if successful. Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// assert!(modbus.set_socket(1337).is_ok());
    /// ```
    pub fn set_socket(&mut self, socket: i32) -> Result<()> {

        unsafe {
            match ffi::modbus_set_socket(self.ctx, socket) {
                -1 => bail!(Error::last_os_error()),
                0 => Ok(()),
                _ => unreachable!(),
            }
        }
    }

    /// `get_socket` - set socket of the context
    ///
    /// The [`get_socket()`](#method.get_socket) function shall return the current socket or file descriptor of the
    /// libmodbus context.
    ///
    /// # Return value
    ///
    /// The function returns a Result containing the current socket or file descriptor of the context if successful.
    /// Otherwise it contains an Error.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let _ = modbus.set_socket(1337).unwrap();
    /// assert_eq!(modbus.get_socket().unwrap(), 1337);
    /// ```
    pub fn get_socket(&self) -> Result<i32> {
        unsafe {
            match ffi::modbus_get_socket(self.ctx) {
                -1 => bail!(Error::last_os_error()),
                socket => Ok(socket),
            }
        }
    }

    /// `get_header_length` - retrieve the current header length
    ///
    /// The [`get_header_length()`](#method.get_header_length) function shall retrieve the current header length from
    /// the backend.
    /// This function is convenient to manipulate a message and so its limited to low-level operations.
    ///
    /// # Return values
    ///
    /// The header length as integer value.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// assert_eq!(modbus.get_header_length(), 7);
    /// ```
    pub fn get_header_length(&self) -> i32 {
        unsafe { ffi::modbus_get_header_length(self.ctx) }
    }

    /// `reply_exception` - send an exception reponse
    ///
    /// The modbus_reply_exception() function shall send an exception response based on the exception_code in argument.
    ///
    /// The libmodbus provides the following exception codes:
    ///
    /// * Modbus::Exception::IllegalFunction  (1)
    /// * Modbus::Exception::IllegalDataAddress  (2)
    /// * Modbus::Exception::IllegalDataValue  (3)
    /// * Modbus::Exception::SlaveOrServerFailure  (4)
    /// * Modbus::Exception::Acknowledge  (5)
    /// * Modbus::Exception::SlaveDeviceBusy  (6)
    /// * Modbus::Exception::NegativeAcknowledge  (7)
    /// * Modbus::Exception::MemoryParity  (8)
    /// * Modbus::Exception::NotDefined  (9)
    /// * Modbus::Exception::GatewayPath (10)
    /// * Modbus::Exception::GatewayTarget (11)
    ///
    /// The initial request `request` is required to build a valid response.
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
    /// assert_eq!(modbus.reply_exception(&request, Exception::Acknowledge as u32).unwrap(), 9);
    /// ```
    pub fn reply_exception(&self, request: &[u8], exception_code: u32) -> Result<i32> {
        unsafe {
            match ffi::modbus_reply_exception(self.ctx, request.as_ptr(), exception_code) {
                -1 => bail!(Error::last_os_error()),
                len => Ok(len),
            }
        }
    }


    /// `strerror`  - return the error message
    ///
    /// The [`strerror()`](#method.strerror) function shall return a message `String` corresponding to the error number
    /// specified by the `errnum` argument.
    ///
    /// ```rust
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// assert_eq!(Modbus::strerror(112345694), "Too many data");
    /// ```
    pub fn strerror(errnum: i32) -> String {

        let c_str = unsafe {
            ::std::ffi::CStr::from_ptr(ffi::modbus_strerror(errnum))
        };
        String::from_utf8_lossy(c_str.to_bytes()).into_owned()
    }



    /// `close` - close a Modbus connection
    ///
    /// The [`close()`](#method.close) function shall close the connection established with the backend set in the
    /// context.
    ///
    /// **It should not nessesary to call these function. Because rusts drop trait handles that for you!**
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// modbus.close();
    /// ```
    pub fn close(&self) {
        unsafe {
            ffi::modbus_close(self.ctx);
        }
    }

    /// `free` - free a libmodbus context
    ///
    /// The [`free()`](#method.free) function shall free an allocated modbus_t structure.
    ///
    /// **It should not nessesary to call these function. Because rusts drop trait handles that for you!**
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// modbus.free();
    /// ```
    pub fn free(&mut self) {
        unsafe {
            ffi::modbus_free(self.ctx);
        }
    }
}

/// `set_bits_from_byte` - set many bits from a single byte value
///
/// The [`set_bits_from_byte()`](#method.set_bits_from_byte) function shall set many bits from a single byte.
/// All 8 bits from the byte value will be written to `dest` array starting at `index` position.
///
/// # Parameters
///
/// * `dest` - destination slice
/// * `index` - starting position where the bit should written
/// * `value` - set many bits from a single byte. All 8 bits from the byte `value` will be written to `dest` slice
/// starting at `index` position.
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
/// use libmodbus_rs::prelude::*;
///
/// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
/// // before
/// assert_eq!(modbus_mapping.get_input_bits_mut(), [0u8, 0, 0, 0, 0]);
///
/// set_bits_from_byte(modbus_mapping.get_input_bits_mut(), 2, 0b1111_1111);
///
/// // after
/// assert_eq!(modbus_mapping.get_input_bits_mut(), [0u8, 0, 1, 1, 1]);
/// ```
pub fn set_bits_from_byte(dest: &mut [u8], index: u32, value: u8) {
    unsafe { ffi::modbus_set_bits_from_byte(dest.as_mut_ptr(), index as c_int, value) }
}

/// `set_bits_from_bytes` -  set many bits from an array of bytes
///
/// The [`set_bits_from_bytes()`](#method.set_bits_from_bytes) function shall set many bits from a single byte.
/// All 8 bits from the byte value will be written to `dest` array starting at index position.
///
/// # Parameters
///
/// * `dest` - destination slice
/// * `index` - starting position where the bit should written
/// * `num_bit`   - how many bits should written
/// * `bytes` - All the bits of the `bytes` parameter, read from the first position of the vec `bytes` are written as
/// bits in the `dest` vec,
///     starting at position `index`
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
/// use libmodbus_rs::prelude::*;
///
/// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
///
/// // before
/// assert_eq!(modbus_mapping.get_input_bits_mut(), [0u8, 0, 0, 0, 0]);
///
/// set_bits_from_bytes(modbus_mapping.get_input_bits_mut(), 0, 2, &[0b0000_1111]);
///
/// // after
/// assert_eq!(modbus_mapping.get_input_bits_mut(), [1u8, 1, 0, 0, 0]);
/// ```
pub fn set_bits_from_bytes(dest: &mut [u8], index: u16, num_bit: u16, bytes: &[u8]) {
    unsafe { ffi::modbus_set_bits_from_bytes(dest.as_mut_ptr(), index as c_int, num_bit as c_uint, bytes.as_ptr()) }
}

/// `get_byte_from_bits` - get the value from many bit
///
/// The [`get_byte_from_bits()`](#method.get_byte_from_bits) function shall extract a value from many bits.
/// All `num_bit` bits from `src` at position `index` will be read as a single value. To obtain a full byte, set `num_bit` to 8.
///
/// # Return value
///
/// The function shall return a byte containing the bits read.
///
/// # Parameters
///
/// * `src`       - bits source
/// * `index`     - starting position where the bit will be read
/// * `num_bit`   - All `num_bit` bits from `src` at position `index` will be read as a single value. To obtain a full
/// byte, set `num_bit` to 8.
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};
/// use libmodbus_rs::prelude::*;
///
/// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
/// let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();
///
/// assert_eq!(get_byte_from_bits(&[0b1111_1111], 0 ,8), 255);
/// ```
pub fn get_byte_from_bits(src: &[u8], index: u8, num_bit: u16) -> u8 {
    unsafe { ffi::modbus_get_byte_from_bits(src.as_ptr(), index as c_int, num_bit as c_uint) }
}

/// `get_float_abcd` - get a float value from 2 registers in `ABCD` byte order
///
/// The [`get_float_abcd()`](#method.get_float_abcd) function shall get a float from 4 bytes in usual Modbus format.
/// The `src` slice mut contain two `u16` values, for example, if the first word is set to `0x0020` and the
/// second to `0xF147`, the float value will be read as `123456.0`.
///
/// # Return value
///
/// The function shall return a float.
///
/// # Parameters
///
/// * `src`   - slice of two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
/// assert_eq!(get_float_abcd(&[0x0020, 0xF147]), 123456.0);
/// ```
pub fn get_float_abcd(src: &[u16]) -> f32 {
    unsafe { ffi::modbus_get_float_abcd(src.as_ptr()) }
}

/// `set_float_abcd` - set a float value in 2 registers using `ABCD` byte order
///
/// The [`set_float_abcd()`](#method.set_float_abcd) function shall set a float to 4 bytes in usual Modbus format.
/// The `dest` slice must contain two `u16` values to be able to store the full result of the conversion.
///
/// # Parameters
///
/// * `src`   - float to 4 bytes (`f32`)
/// * `dest`  - slice must contain two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// let mut dest = vec![0; 2];
/// set_float_abcd(123456.0, &mut dest);
/// assert_eq!(&dest, &[0x0020, 0xF147]);
/// ```
pub fn set_float_abcd(src: f32, dest: &mut [u16]) {
    // &mut [u16; 2] is not working here
    unsafe { ffi::modbus_set_float_abcd(src, dest.as_mut_ptr()) }
}

/// `get_float_badc` - get a float value from 2 registers in `BADC` byte order
///
/// The [`get_float_badc()`](#method.get_float_badc) function shall get a float from 4 bytes with swapped bytes (`BADC`
/// instead of `ABCD`).
/// The `src` slice mut contain two `u16` values, for example, if the first word is set to `0x2000` and the second to
/// `0x47F1`, the float value will be read as `123456.0`.
///
/// # Return value
///
/// The function shall return a float.
///
/// # Parameters
///
/// * `src`   - slice of two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// assert_eq!(get_float_badc(&[0x2000, 0x47F1]), 123456.0);
/// ```
pub fn get_float_badc(src: &[u16]) -> f32 {
    unsafe { ffi::modbus_get_float_badc(src.as_ptr()) }
}

/// `set_float_badc` - set a float value in 2 registers using `BADC` byte order
///
/// The [`set_float_badc()`](#method.set_float_badc) function shall set a float to 4 bytes in swapped bytes Modbus
/// format (`BADC` insted of `ABCD`).
/// The dest slice must contain two `u16` values to be able to store the full result of the conversion.
///
/// # Parameters
///
/// * `src`   - float to 4 bytes (`f32`)
/// * `dest`  - slice must contain two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// let mut dest = vec![0; 2];
/// set_float_badc(123456.0, &mut dest);
/// assert_eq!(&dest, &[0x2000, 0x47F1]);
/// ```
pub fn set_float_badc(src: f32, dest: &mut [u16]) {
    // &mut [u16; 2] is not working here
    unsafe { ffi::modbus_set_float_badc(src, dest.as_mut_ptr()) }
}

/// `get_float_cdab` - get a float value from 2 registers in `CDAB` byte order
///
/// The [`get_float_cdab()`](#method.get_float_cdab) function shall get a float from 4 bytes with swapped bytes (`CDAB`
/// instead of `ABCD`).
/// The `src` slice mut contain two `u16` values, for example, if the first word is set to `0x2000` and the second to
/// `0x47F1`, the float value will be read as `123456.0`.
///
/// # Return value
///
/// The function shall return a float.
///
/// # Parameters
///
/// * `src`   - slice of two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// assert_eq!(get_float_cdab(&[0xF147, 0x0020]), 123456.0);
/// ```
pub fn get_float_cdab(src: &[u16]) -> f32 {
    unsafe { ffi::modbus_get_float_cdab(src.as_ptr()) }
}

/// `set_float_cdab` - set a float value in 2 registers using `CDAB` byte order
///
/// The [`set_float_cdab()`](#method.set_float_cdab) function shall set a float to 4 bytes in swapped bytes Modbus
/// format (`CDAB` insted of `ABCD`).
/// The `dest` slice must contain two `u16` values to be able to store the full result of the conversion.
///
/// # Parameters
///
/// * `src`   - float to 4 bytes (`f32`)
/// * `dest`  - slice must contain two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// let mut dest = vec![0; 2];
/// set_float_cdab(123456.0, &mut dest);
/// assert_eq!(&dest, &[0xF147, 0x0020]);
/// ```
pub fn set_float_cdab(src: f32, dest: &mut [u16]) {
    // &mut [u16; 2] is not working here
    unsafe { ffi::modbus_set_float_cdab(src, dest.as_mut_ptr()) }
}

/// `get_float_dcba` - get a float value from 2 registers in `DCBA` byte order
///
/// The [`get_float_dcba()`](#method.get_float_dcba) function shall get a float from 4 bytes with swapped bytes (`DCBA`
/// instead of `ABCD`).
/// The src slice mut contain two `u16` values, for example, if the first word is set to `0x2000` and the second to
/// `0x47F1`, the float value will be read as `123456.0`.
///
/// # Return value
///
/// The function shall return a float.
///
/// # Parameters
///
/// * `src`   - slice of two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// assert_eq!(get_float_dcba(&[0x47F1, 0x2000]), 123456.0);
/// ```
pub fn get_float_dcba(src: &[u16]) -> f32 {
    unsafe { ffi::modbus_get_float_dcba(src.as_ptr()) }
}

/// `set_float_dcba` - set a float value in 2 registers using `DCBA` byte order
///
/// The [`set_float_dcba()`](#method.set_float_dcba) function shall set a float to 4 bytes in swapped bytes Modbus
/// format (`DCBA` insted of `ABCD`).
/// The `dest` slice must contain two `u16` values to be able to store the full result of the conversion.
///
/// # Parameters
///
/// * `src`   - float to 4 bytes (`f32`)
/// * `dest`  - slice must contain two `u16` values
///
/// # Examples
///
/// ```rust
/// use libmodbus_rs::prelude::*;
///
/// let mut dest = vec![0; 2];
/// set_float_dcba(123456.0, &mut dest);
/// assert_eq!(&dest, &[0x47F1, 0x2000]);
/// ```
pub fn set_float_dcba(src: f32, dest: &mut [u16]) {
    // &mut [u16; 2] is not working here
    unsafe { ffi::modbus_set_float_dcba(src, dest.as_mut_ptr()) }
}


impl Drop for Modbus {
    fn drop(&mut self) {
        self.close();
        self.free();
    }
}
