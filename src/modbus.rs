
use libc::{c_char, c_int};
use std::ffi::CString;
use std::str;
use std::io::{Error, ErrorKind};
use libmodbus_sys;


pub struct Modbus {
    ctx: *mut libmodbus_sys::modbus_t,
}

impl Modbus {
    /// create a libmodbus context for RTU
    ///
    /// The modbus_new_rtu() function shall allocate and initialize a modbus_t structure to communicate
    /// in RTU mode on a serial line.
    ///
    /// The device argument specifies the name of the serial port handled by the OS, eg. "/dev/ttyS0" or "/dev/ttyUSB0".
    /// On Windows, it’s necessary to prepend COM name with "\\.\" for COM number greater than 9,
    /// eg. "\\\\.\\COM10". See http://msdn.microsoft.com/en-us/library/aa365247(v=vs.85).aspx for details
    /// The baud argument specifies the baud rate of the communication, eg. 9600, 19200, 57600, 115200, etc.
    ///
    /// The parity argument can have one of the following values
    ///     * N for none
    ///     * E for even
    ///     * O for odd
    ///
    ///    The data_bits argument specifies the number of bits of data, the allowed values are 5, 6, 7 and 8.
    ///    The stop_bits argument specifies the bits of stop, the allowed values are 1 and 2.
    ///    Once the modbus_t structure is initialized, you must set the slave of your device with
    ///    modbus_set_slave(3) and connect to the serial bus with modbus_connect(3).
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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
                   stop_bit: i32) -> Result<Modbus, Error> {
        unsafe {
            let device = CString::new(device).unwrap();
            let ctx = libmodbus_sys::modbus_new_rtu(device.as_ptr(),
                baud as c_int,
                parity as c_char,
                data_bit as c_int,
                stop_bit as c_int);

            if ctx.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }

    /// get the current serial mode
    ///
    /// The modbus_rtu_get_serial_mode() function shall return the serial mode currently
    /// used by the libmodbus context:
    /// MODBUS_RTU_RS232
    ///     the serial line is set for RS232 communication. RS-232 (Recommended Standard 232)
    ///     is the traditional name for a series of standards for serial binary single-ended
    ///     data and control signals connecting between a DTE (Data Terminal Equipment) and a
    ///     DCE (Data Circuit-terminating Equipment). It is commonly used in computer serial ports
    /// MODBUS_RTU_RS485
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
    /// use modbus_rs::Modbus;
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// // assert_eq!(modbus.rtu_get_serial_mode(), SerialMode::MODBUS_RTU_RS232);
    /// assert_eq!(modbus.rtu_get_serial_mode(), 0);
    /// ```
    pub fn rtu_get_serial_mode(&self) -> i32 {
        unsafe {
            libmodbus_sys::modbus_rtu_get_serial_mode(self.ctx)
        }
    }

    /// create a libmodbus context for TCP/IPv4
    ///
    /// The modbus_new_tcp() function shall allocate and initialize a modbus_t structure
    /// to communicate with a Modbus TCP IPv4 server.
    /// The ip argument specifies the IP address of the server to which the client wants to
    /// establish a connection. A NULL value can be used to listen any addresses in server mode.
    /// The port argument is the TCP port to use. Set the port to MODBUS_TCP_DEFAULT_PORT
    /// to use the default one (502). It’s convenient to use a port number greater than or
    /// equal to 1024 because it’s not necessary to have administrator privileges.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn new_tcp(ip: &str,
                    port: i32) -> Result<Modbus, Error> {
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

    /// establish a Modbus connection
    ///
    /// The modbus_connect() function shall establish a connection to a Modbus server,
    /// a network or a bus using the context information of libmodbus context given in argument.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    pub fn connect(&self) -> Result<(), Error> {
        unsafe {
            match libmodbus_sys::modbus_connect(self.ctx) {
                -1 => Err(Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    /// set slave number in the context
    ///
    /// The modbus_set_slave() function shall set the slave number in the libmodbus context.
    /// The behavior depends of network and the role of the device:
    /// RTU
    /// Define the slave ID of the remote device to talk in master mode or set the internal slave ID in slave mode. According to the protocol, a Modbus device must only accept message holding its slave number or the special broadcast number.
    /// TCP
    /// The slave number is only required in TCP if the message must reach a device on a serial network. Some not compliant devices or software (such as modpoll) uses the slave ID as unit identifier, that’s incorrect (cf page 23 of Modbus Messaging Implementation Guide v1.0b) but without the slave value, the faulty remote device or software drops the requests! The special value MODBUS_TCP_SLAVE (0xFF) can be used in TCP mode to restore the default value.
    /// The broadcast address is MODBUS_BROADCAST_ADDRESS. This special value must be use when you want all Modbus devices of the network receive the request.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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
                -1 => Err(Error::last_os_error()),
                _ => Ok(()),
            }
        }
    }

    /// close a Modbus connection
    ///
    /// The modbus_close() function shall close the connection established with the backend set in the context.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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

    /// free a libmodbus context
    ///
    /// The modbus_free() function shall free an allocated modbus_t structure.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::Modbus;
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
