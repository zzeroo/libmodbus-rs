use raw::*;
use libc::{c_char};

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

    
}



#[cfg(test)]
mod tests {
    use super::Modbus;

    #[test]
    fn crate_modbus() {
        let modbus = Modbus::new_rtu("/dev/ttyUSB0", 9600, 'N', 8, 1);
    }

}
