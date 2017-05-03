use error::ModbusError;
use libc::{c_char, c_int};
use libmodbus_sys;
use std::ffi::CString;
use std::io::{Error, ErrorKind};
use std::str;


#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum SerialMode {
    MODBUS_RTU_RS232 = libmodbus_sys::MODBUS_RTU_RS232 as isize,
    MODBUS_RTU_RS485 = libmodbus_sys::MODBUS_RTU_RS485 as isize,
}

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum RTUMode {
    MODBUS_RTU_RTS_NONE = libmodbus_sys::MODBUS_RTU_RTS_NONE as isize,
    MODBUS_RTU_RTS_UP = libmodbus_sys::MODBUS_RTU_RTS_UP as isize,
    MODBUS_RTU_RTS_DOWN = libmodbus_sys::MODBUS_RTU_RTS_DOWN as isize,
}

pub struct Modbus {
    ctx: *mut libmodbus_sys::modbus_t,
}

impl Modbus {
    /// `new_rtu` - create a libmodbus context for RTU
    ///
    /// The [`new_rtu()`](#method.new_rtu) function shall allocate and initialize a structure
    /// to communicate in RTU mode on a serial line.
    ///
    /// The **device** argument specifies the name of the serial port handled by the OS, eg. "/dev/ttyS0" or "/dev/ttyUSB0".
    /// On Windows, it’s necessary to prepend COM name with "\\.\" for COM number greater than 9,
    /// eg. "\\\\.\\COM10". See http://msdn.microsoft.com/en-us/library/aa365247(v=vs.85).aspx for details
    /// The **baud** argument specifies the baud rate of the communication, eg. 9600, 19200, 57600, 115200, etc.
    ///
    /// The **parity** argument can have one of the following values
    ///     * N for none
    ///     * E for even
    ///     * O for odd
    ///
    ///    The **data_bits argument** specifies the number of bits of data, the allowed values are 5, 6, 7 and 8.
    ///    The **stop_bits** argument specifies the bits of stop, the allowed values are 1 and 2.
    ///    Once the modbus structure is initialized, you must set the slave of your device with
    ///    [`set_slave()`](#method.set_slave) and connect to the serial bus with [`connect()`](#method.connect).
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
    ///
    /// const YOUR_DEVICE_ID: i32 = 1;
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    /// modbus.set_slave(YOUR_DEVICE_ID);
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn new_rtu(device: &str,
                   baud: i32,
                   parity: char,
                   data_bit: i32,
                   stop_bit: i32) -> Result<Modbus, ModbusError> {
        unsafe {
            let device = CString::new(device).unwrap();
            let ctx = libmodbus_sys::modbus_new_rtu(device.as_ptr(),
                baud as c_int,
                parity as c_char,
                data_bit as c_int,
                stop_bit as c_int);

            if ctx.is_null() {
                Err(ModbusError::InvalArg)
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }

    /// `rtu_get_serial_mode` - get the current serial mode
    ///
    /// The [`rtu_get_serial_mode()`](#method.rtu_get_serial_mode) function shall return the serial mode currently
    /// used by the libmodbus context:
    ///
    /// `SerialMode::MODBUS_RTU_RS232`
    ///     the serial line is set for RS232 communication. RS-232 (Recommended Standard 232)
    ///     is the traditional name for a series of standards for serial binary single-ended
    ///     data and control signals connecting between a DTE (Data Terminal Equipment) and a
    ///     DCE (Data Circuit-terminating Equipment). It is commonly used in computer serial ports
    ///
    /// `SerialMode::MODBUS_RTU_RS485`
    ///     the serial line is set for RS485 communication.
    ///     EIA-485, also known as TIA/EIA-485 or RS-485, is a standard defining the electrical
    ///     characteristics of drivers and receivers for use in balanced digital multipoint systems.
    ///     This standard is widely used for communications in industrial automation because it can be
    ///     used effectively over long distances and in electrically noisy environments.
    ///
    /// This function is only available on Linux kernels 2.6.28 onwards
    /// and can only be used with a context using a RTU backend.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, SerialMode};
    ///
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::MODBUS_RTU_RS232);
    /// ```
    pub fn rtu_get_serial_mode(&self) -> Result<SerialMode, ModbusError> {
        unsafe {
            let mode = libmodbus_sys::modbus_rtu_get_serial_mode(self.ctx);
            match mode {
                mode if mode == SerialMode::MODBUS_RTU_RS232 as i32 => Ok(SerialMode::MODBUS_RTU_RS232),
                mode if mode == SerialMode::MODBUS_RTU_RS485 as i32 => Ok(SerialMode::MODBUS_RTU_RS485),
                _ => Err(ModbusError::InvalArg),
            }
        }
    }

    /// `rtu_set_serial_mode` - set the serial mode
    ///
    /// The [`rtu_set_serial_mode()`](#method.rtu_set_serial_mode) function shall set the selected serial mode:
    ///
    /// MODBUS_RTU_RS232
    ///     the serial line is set for RS232 communication.
    ///     RS-232 (Recommended Standard 232) is the traditional name for a series of
    ///     standards for serial binary single-ended data and control signals connecting
    ///     between a DTE (Data Terminal Equipment) and a DCE (Data Circuit-terminating Equipment).
    ///     It is commonly used in computer serial ports
    ///
    /// MODBUS_RTU_RS485
    ///     the serial line is set for RS485 communication.
    ///     EIA-485, also known as TIA/EIA-485 or RS-485, is a standard defining the
    ///     electrical characteristics of drivers and receivers for use in balanced digital multipoint systems.
    ///     This standard is widely used for communications in industrial automation
    ///     because it can be used effectively over long distances and in electrically noisy environments.
    ///
    /// This function is only supported on Linux kernels 2.6.28 onwards.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, SerialMode};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// assert!(modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS232).is_ok());
    /// ```
    pub fn rtu_set_serial_mode(&mut self, mode: SerialMode) -> Result<i32, ModbusError> {
        unsafe {
            match libmodbus_sys::modbus_rtu_set_serial_mode(self.ctx, mode as c_int) {
                -1 => Err(ModbusError::NotRTU),
                _ => Ok(0),
            }
        }
    }

    /// `rtu_set_rts` - set the RTS mode in RTU
    ///
    /// The [`rtu_set_rts()`](#method.rtu_set_rts) function shall set the Request To Send mode to communicate on a RS485 serial bus.
    /// By default, the mode is set to `RTUMode::MODBUS_RTU_RTS_NONE` and no signal is issued before writing data on the wire.
    ///
    /// To enable the RTS mode, the values `RTUMode::MODBUS_RTU_RTS_UP` or `RTUMode::MODBUS_RTU_RTS_DOWN` must be used,
    /// these modes enable the RTS mode and set the polarity at the same time. When `RTUMode::MODBUS_RTU_RTS_UP` is used,
    /// an ioctl call is made with RTS flag enabled then data is written on the bus after a delay of 1 ms,
    /// then another ioctl call is made with the RTS flag disabled and again a delay of 1 ms occurs.
    /// The `RTUMode::MODBUS_RTU_RTS_DOWN` mode applies the same procedure but with an inverted RTS flag.
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, SerialMode, RTUMode};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// match modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS485) {
    ///     Ok(_) => assert!(modbus.rtu_set_rts(RTUMode::MODBUS_RTU_RTS_UP).is_ok()),
    ///     Err(_) => {}
    /// };
    /// ```
    pub fn rtu_set_rts(&mut self, mode: RTUMode) -> Result<i32, Error> {
        unsafe {
            match libmodbus_sys::modbus_rtu_set_rts(self.ctx, mode as c_int) {
                -1 => Err(Error::last_os_error()),
                _ => Ok(0),
            }
        }
    }

    /// `new_tcp` - create a libmodbus context for TCP/IPv4
    ///
    /// The [`new_tcp()`](#method.new_tcp) function shall allocate and initialize a modbus_t structure
    /// to communicate with a Modbus TCP IPv4 server.
    /// The **ip** argument specifies the IP address of the server to which the client wants to
    /// establish a connection. A empty string `""` value can be used to listen any addresses in server mode.
    /// The **port** argument is the TCP port to use. Set the port to `MODBUS_TCP_DEFAULT_PORT`
    /// to use the default one (502). It’s convenient to use a port number greater than or
    /// equal to 1024 because it’s not necessary to have administrator privileges.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, MODBUS_TCP_DEFAULT_PORT};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus = Modbus::new_tcp("127.0.0.1", MODBUS_TCP_DEFAULT_PORT).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn new_tcp(ip: &str,
                    port: u32) -> Result<Modbus, Error> {
        unsafe {
            let ip = CString::new(ip).unwrap();
            let ctx = libmodbus_sys::modbus_new_tcp(ip.as_ptr(),
                port as c_int);

            if ctx.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }

    /// `connect` - establish a Modbus connection
    ///
    /// The [`connect()`](#method.connect) function shall establish a connection to a Modbus server,
    /// a network or a bus.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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
    /// use modbus_rs::Modbus;
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
    pub fn set_slave(&mut self, slave: i32) -> Result<(), Error> {
        unsafe {
            match libmodbus_sys::modbus_set_slave(self.ctx, slave as c_int) {
                -1 => Err(Error::new(ErrorKind::Other, "The slave number is invalid.")),
                _ => Ok(()),
            }
        }
    }

    /// `close` - close a Modbus connection
    ///
    /// The [`close()`](#method.close) function shall close the connection established with the backend set in the context.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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
    /// use modbus_rs::Modbus;
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
