use raw::*;
use libc::{c_char, c_int};
pub use error::Error;

// https://doc.rust-lang.org/book/error-handling.html#the-result-type-alias-idiom
pub type Result<T> = ::std::result::Result<T, Error>;

/// This struct holds the current context `ctx`
///
/// This logic is derived from that one libmodbus uses.
#[derive(Debug, Eq, PartialEq)]
pub struct Modbus { ctx: *mut modbus_t }

/// I love rust. It's so expressive.
impl Drop for Modbus {
    fn drop(&mut self) {
        self.close();
        self.free();
    }
}

impl Modbus {
    /// Creates a new modbus context with the RTU backend
    ///
    /// # Attributes
    /// * `device`         - Device string e.g. "/dev/ttyUSB0"
    /// * `baud`           - Baud rate
    /// * `parity`         - Parity
    /// * `data_bit`       - Number of data bits
    /// * `stop_bit`       - Number of stop bits
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::Modbus;
    ///
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// ```
    pub fn new_rtu(device: &str, baud: i32, parity: char, data_bit: i32, stop_bit: i32) -> Self {
        Modbus {
            ctx: {
                unsafe {
                    ::raw::modbus_new_rtu(::std::ffi::CString::new(device).unwrap().as_ptr(), baud, parity as c_char, data_bit, stop_bit)
                }
            }
        }
    }

    /// Define the slave ID of the remote device to talk in master mode or set the
    /// internal slave ID in slave mode
    ///
    /// # Attributes
    /// * `slave_id`    - New modbus slave id (valid range: `>= 0 && <= 247`), `0` is broadcast address
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `Error::InvalidSlaveID` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus, Error};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// assert_eq!(modbus.set_slave(10), Ok(0));
    /// assert_eq!(modbus.set_slave(255), Err(Error::InvalidSlaveID));
    /// ```
    pub fn set_slave(&mut self, slave_id: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_set_slave(self.ctx, slave_id) {
                -1 => Err(Error::InvalidSlaveID),
                _ => Ok(0),
            }
        }
    }

    /// Set debug flag of the context
    ///
    /// # Attributes
    /// * `flag`    - boolean `true` or `false`
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `Error::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// assert_eq!(modbus.set_debug(true), Ok(0));
    /// ```
    pub fn set_debug(&mut self, flag: bool) -> Result<i32> {
        unsafe {
            match ::raw::modbus_set_debug(self.ctx, flag as c_int) {
                -1 => Err(Error::InvalidDebug),
                _ => Ok(0),
            }
        }
    }

    /// Set the serial mode
    ///
    /// # Attributes
    /// * `mode`    - serial mode
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `Error::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// ```
    pub fn rtu_set_serial_mode(&mut self, mode: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_rtu_set_serial_mode(self.ctx, mode) {
                -1 => Err(Error::InvalidRTUSerialMode),
                _ => Ok(0),
            }
        }
    }

    /// Set the RTS mode in RTU
    ///
    /// # Attributes
    /// * `mode`    - serial mode
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `Error::GenericError` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// assert_eq!(modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN), Ok(0));
    /// ```
    pub fn rtu_set_rts(&mut self, mode: i32) -> Result<i32> {
        unsafe {
            match ::raw::modbus_rtu_set_rts(self.ctx, mode) {
                -1 => Err(Error::InvalidRTURTS),
                _ => Ok(0),
            }
        }
    }

    /// Establish a Modbus connection
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `Error::GenericError` is returned
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// assert_eq!(modbus.connect(), Ok(0));
    /// ```
    pub fn connect(&self) -> Result<i32> {
        unsafe {
            match ::raw::modbus_connect(self.ctx) {
                -1 => Err(Error::ConnectionError),
                _ => Ok(0),
            }
        }
    }

    /// modbus_close - close a Modbus connection
    ///
    /// The `modbus_close()` function shall close the connection established with the backend set in the context.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(1);
    /// match modbus.connect() {
    ///     Err(_) => { modbus.close(); }
    ///     Ok(_) => {
    ///         let _ = modbus.read_registers(0, 1);
    ///     }
    /// }
    /// ```
    pub fn close(&self) {
        unsafe {
            ::raw::modbus_close(self.ctx);
        }
    }

    /// Free a libmodbus context
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// modbus.free(); 
    /// ```
    pub fn free(&self) {
        unsafe {
            ::raw::modbus_free(self.ctx);
        }
    }

    /// Read many registers
    ///
    /// # Attributes
    /// * `address`    - Start address from which the read should start
    /// * `num_reg`    - Number of holding registers to read from `address` of the remote device
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
    /// let _ = modbus.connect(); 
    /// let mut tab_reg: Vec<u16> = modbus.read_registers(0, 19).unwrap();
    /// ```
    pub fn read_registers(&self, address: i32, num_reg: i32) -> Result<Vec<u16>> {
        let mut tab_reg = vec![0u16; num_reg as usize];
        unsafe {
            match ::raw::modbus_read_registers(self.ctx, address, num_reg, tab_reg.as_mut_ptr()) {
                -1 => { Err(Error::ConnectionError)}
                _ => { Ok(tab_reg) }
            }
        }
    }

    /// Read many registers
    ///
    /// # Attributes
    /// * `address`    - Start address from which the read should start
    /// * `num_reg`    - Number of holding registers to read from `address` of the remote device
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.rtu_set_rts(libmodbus_rs::MODBUS_RTU_RTS_DOWN);
    /// let _ = modbus.connect(); 
    /// let mut tab_reg: Vec<u16> = modbus.read_input_registers(0, 19).unwrap();
    /// ```
    pub fn read_input_registers(&self, address: i32, num_reg: i32) -> Result<Vec<u16>> {
        let mut tab_reg = vec![0u16; num_reg as usize];
        unsafe {
            match ::raw::modbus_read_input_registers(self.ctx, address, num_reg, tab_reg.as_mut_ptr()) {
                -1 => Err(Error::ReadFailure),
                _ => Ok(tab_reg),
            }
        }
    }

    /// read many bits
    ///
    /// This function shall read the state of `num_bits` (coils) beginning from the `address` of
    /// the remote device. The return value is an vector of u8's encapsuled in a Result.
    ///
    /// This funkciont uses the Modbus function **01 (0x01) Read Coils**
    ///
    /// # Attributes
    /// * `address`     - Start Adresse from where the read should begin
    /// * `num_bits`    - Number of bit to be read
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// let _ = modbus.read_bits(0, 1).unwrap();
    /// ```
    pub fn read_bits(&self, address: i32, num_bits: i32) -> Result<Vec<u8>> {
        let mut coils = vec![0u8; num_bits as usize];
        unsafe {
            match ::raw::modbus_read_bits(self.ctx, address, num_bits, coils.as_mut_ptr()) {
                -1 => Err(Error::ReadFailure),
                _ => Ok(coils),
            }
        }
    }


    /// write a single coil
    ///
    /// # Attributes
    /// * `address`     - Coil address
    /// * `state`      - Coil state
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// let _ = modbus.write_bit(0, true).unwrap();
    /// ```
    pub fn write_bit(&self, address: i32, state: bool) -> Result<()> {
        unsafe {
            match ::raw::modbus_write_bit(self.ctx, address, state as i32) {
                -1 => Err(Error::WriteFailure),
                _ => Ok(())
            }
        }
    }

    /// write a single register
    ///
    /// # Attributes
    /// * `address`     - Register address
    /// * `value`       - New register value
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// let _ = modbus.write_register(0, 0x1234);
    /// let tab_reg: Vec<u16> = modbus.read_registers(0, 1).unwrap();
    /// assert_eq!(tab_reg, &[0x1234]);
    /// ```
    pub fn write_register(&self, address: i32, value: i32) -> Result<()> {
        let result = unsafe {
            ::raw::modbus_write_register(self.ctx, address, value)
        };
        match result {
            1 => Ok(()),
            _ => Err(Error::WriteFailure),
        }
    }

    /// write multiple registers
    ///
    /// # Attributes
    /// * `address`     - Start address register
    /// * `register`    - Vector of u16 values
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// let register = [0x1111u16, 0x2222u16];
    /// assert_eq!(modbus.write_registers(0, &register), Ok(2));
    /// ```
    pub fn write_registers(&self, address: i32, register: &[u16]) -> Result<i32> {
        let result = unsafe {
            ::raw::modbus_write_registers(self.ctx, address, register.len() as i32, register.as_ptr())
        };
        match result {
            -1 => Err(Error::WriteFailure),
            num => Ok(num),
        }
    }

    /// send a raw request
    ///
    /// # Attributes
    /// * `raw_request` - Vector of u8 values
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// let _ = modbus.set_slave(46);
    /// let _ = modbus.connect(); 
    /// let raw_request =  vec![0x2E, 0x08, 0x00, 0x01];
    /// assert_eq!(modbus.send_raw_request(&raw_request), Ok(6));
    /// ```
    pub fn send_raw_request(&self, raw_request: &[u8]) -> Result<i32> {
        let result = unsafe {
            ::raw::modbus_send_raw_request(self.ctx, raw_request.as_ptr(), raw_request.len() as i32)
        };
        match result {
            -1 => Err(Error::WriteFailure),
            num => Ok(num),
        }
    }
    

}



#[cfg(test)]
mod tests {
    use super::Modbus;

    #[test]
    fn crate_modbus() {
        let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
        assert_eq!(modbus, modbus);
    }

}
