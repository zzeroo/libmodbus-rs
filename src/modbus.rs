use raw::*;
use libc::{c_char};

#[derive(Debug, Eq, PartialEq)]
pub enum ModbusError {
    INVALID_SLAVE_ID,
}

pub struct Modbus { ctx: *mut modbus_t }

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
    /// * `slave_id`    - New modbus slave id (valid range: `>= 0 && <= 247`)
    ///
    /// # Examples
    /// A `Ok(0)` signals all right, on error a `ModbusError::INVALID_SLAVE_ID` is returned
    ///
    /// ```
    /// use libmodbus_rs::modbus::{Modbus, ModbusError};
    ///
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    /// assert_eq!(modbus.set_slave(10), Ok(0));
    /// assert_eq!(modbus.set_slave(255), Err(ModbusError::INVALID_SLAVE_ID));
    /// ```
    pub fn set_slave(&mut self, slave_id: i32) -> Result<i32, ModbusError> {
        unsafe {
            let ret: i32 = ::raw::modbus_set_slave(self.ctx, slave_id);
            if ret == -1 {
                return Err(ModbusError::INVALID_SLAVE_ID);
            } else {
                return Ok(ret);
            }
        }
    }


}



#[cfg(test)]
mod tests {
    use super::Modbus;

    #[test]
    fn crate_modbus() {
        let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    }

}
