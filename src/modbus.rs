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
