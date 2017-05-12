use errors::*;
use libc::{c_char, c_int};
use libmodbus_sys;
use modbus::Modbus;
use std::ffi::CString;
use std::str;


#[derive(Debug, PartialEq)]
pub enum SerialMode {
    MODBUS_RTU_RS232 = libmodbus_sys::MODBUS_RTU_RS232 as isize,
    MODBUS_RTU_RS485 = libmodbus_sys::MODBUS_RTU_RS485 as isize,
}

#[derive(Debug, PartialEq)]
pub enum RequestToSendMode {
    MODBUS_RTU_RTS_NONE = libmodbus_sys::MODBUS_RTU_RTS_NONE as isize,
    MODBUS_RTU_RTS_UP = libmodbus_sys::MODBUS_RTU_RTS_UP as isize,
    MODBUS_RTU_RTS_DOWN = libmodbus_sys::MODBUS_RTU_RTS_DOWN as isize,
}

/// The RTU backend (Remote Terminal Unit) is used in serial communication and makes use of a compact, binary representation of the data for protocol communication.
/// The RTU format follows the commands/data with a cyclic redundancy check checksum as an error check mechanism to ensure the reliability of data.
/// Modbus RTU is the most common implementation available for Modbus. A Modbus RTU message must be transmitted continuously without inter-character hesitations
/// (extract from Wikipedia, Modbus, http://en.wikipedia.org/wiki/Modbus (as of Mar. 13, 2011, 20:51 GMT).
///
/// The Modbus RTU framing calls a slave, a device/service which handle Modbus requests, and a master, a client which send requests. The communication is always initiated by the master.
///
/// Many Modbus devices can be connected together on the same physical link so before sending a message, you must set the slave (receiver) with modbus_set_slave(3).
/// If you’re running a slave, its slave number will be used to filter received messages.
///
/// The libmodbus implementation of RTU isn’t time based as stated in original Modbus specification,
/// instead all bytes are sent as fast as possible and a response or an indication is considered complete when all expected characters have been received.
/// This implementation offers very fast communication but you must take care to set a response timeout of slaves less than response timeout of master
/// (ortherwise other slaves may ignore master requests when one of the slave is not responding).
///
/// * Create a Modbus RTU context
///     - [`new_rtu()`](struct.Modbus.html#method.new_rtu)
///
/// * Set the serial mode
///     - [`rtu_get_serial_mode()`](struct.Modbus.html#method.rtu_get_serial_mode), [`rtu_set_serial_mode()`](struct.Modbus.html#method.rtu_set_serial_mode), [`rtu_get_rts()`](struct.Modbus.html#method.rtu_get_rts), [`rtu_set_rts()`](struct.Modbus.html#method.rtu_set_rts), [`rtu_set_custom_rts()`](struct.Modbus.html#method.rtu_set_custom_rts), [`rtu_get_rts_delay()`](struct.Modbus.html#method.rtu_get_rts_delay), [`rtu_set_rts_delay()`](struct.Modbus.html#method.rtu_set_rts_delay)
///
pub trait ModbusRTU {
    fn new_rtu(device: &str,
                   baud: i32,
                   parity: char,
                   data_bit: i32,
                   stop_bit: i32) -> Result<Modbus>;
   fn rtu_get_serial_mode(&self) -> Result<SerialMode>;
   fn rtu_set_serial_mode(&mut self, mode: SerialMode) -> Result<SerialMode>;
   fn rtu_get_rts(&self) -> Result<RequestToSendMode>;
   fn rtu_set_rts(&mut self, mode: RequestToSendMode) -> Result<RequestToSendMode>;
   fn rtu_set_custom_rts(&mut self, _mode: RequestToSendMode) -> Result<i32>;
   fn rtu_get_rts_delay(&self) -> Result<i32>;
   fn rtu_set_rts_delay(&mut self, us: i32) -> Result<i32>;
}

impl ModbusRTU for Modbus {
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
    /// The **parity** argument can have one of the following values:
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
    /// use libmodbus_rs::{Modbus, ModbusRTU};
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
    fn new_rtu(device: &str,
                   baud: i32,
                   parity: char,
                   data_bit: i32,
                   stop_bit: i32) -> Result<Modbus> {
        unsafe {
            let device = CString::new(device).unwrap();
            let ctx = libmodbus_sys::modbus_new_rtu(device.as_ptr(),
                baud as c_int,
                parity as c_char,
                data_bit as c_int,
                stop_bit as c_int);

            if ctx.is_null() {
                Err("Could not create new RTU context".into())
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
    /// use libmodbus_rs::{Modbus, ModbusRTU, SerialMode};
    ///
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::MODBUS_RTU_RS232);
    /// ```
    fn rtu_get_serial_mode(&self) -> Result<SerialMode> {
        unsafe {
            let mode = libmodbus_sys::modbus_rtu_get_serial_mode(self.ctx);
            match mode {
                mode if mode == SerialMode::MODBUS_RTU_RS232 as i32 => Ok(SerialMode::MODBUS_RTU_RS232),
                mode if mode == SerialMode::MODBUS_RTU_RS485 as i32 => Ok(SerialMode::MODBUS_RTU_RS485),
                _ => Err("Could not get RTU serial mode".into()),
            }
        }
    }

    /// `rtu_set_serial_mode` - set the serial mode
    ///
    /// The [`rtu_set_serial_mode()`](#method.rtu_set_serial_mode) function shall set the selected serial mode:
    ///
    /// `MODBUS_RTU_RS232`
    ///     the serial line is set for RS232 communication.
    ///     RS-232 (Recommended Standard 232) is the traditional name for a series of
    ///     standards for serial binary single-ended data and control signals connecting
    ///     between a DTE (Data Terminal Equipment) and a DCE (Data Circuit-terminating Equipment).
    ///     It is commonly used in computer serial ports
    ///
    /// `MODBUS_RTU_RS485`
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
    /// use libmodbus_rs::{Modbus, ModbusRTU, SerialMode};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// assert!(modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS232).is_ok());
    /// ```
    fn rtu_set_serial_mode(&mut self, mode: SerialMode) -> Result<SerialMode> {
        unsafe {
            let mode = libmodbus_sys::modbus_rtu_set_serial_mode(self.ctx, mode as c_int) as u32;
            match mode {
                libmodbus_sys::MODBUS_RTU_RS232 => Ok(SerialMode::MODBUS_RTU_RS232),
                libmodbus_sys::MODBUS_RTU_RS485 => Ok(SerialMode::MODBUS_RTU_RS485),
                _ => Err("Could not set RTU serial mode".into()),
            }
        }
    }

    /// `rtu_set_rts` - set the RTS mode in RTU
    ///
    /// The [`rtu_set_rts()`](#method.rtu_set_rts) function shall set the Request To Send mode to communicate on a RS485 serial bus.
    /// By default, the mode is set to `RequestToSendMode::MODBUS_RTU_RTS_NONE` and no signal is issued before writing data on the wire.
    ///
    /// To enable the RTS mode, the values `RequestToSendMode::MODBUS_RTU_RTS_UP` or `RequestToSendMode::MODBUS_RTU_RTS_DOWN` must be used,
    /// these modes enable the RTS mode and set the polarity at the same time. When `RequestToSendMode::MODBUS_RTU_RTS_UP` is used,
    /// an ioctl call is made with RTS flag enabled then data is written on the bus after a delay of 1 ms,
    /// then another ioctl call is made with the RTS flag disabled and again a delay of 1 ms occurs.
    /// The `RequestToSendMode::MODBUS_RTU_RTS_DOWN` mode applies the same procedure but with an inverted RTS flag.
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusRTU, SerialMode, RequestToSendMode};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// let serial_mode = modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS485);
    ///
    /// assert!(modbus.rtu_set_rts(RequestToSendMode::MODBUS_RTU_RTS_UP).is_ok());
    /// ```
    fn rtu_set_rts(&mut self, mode: RequestToSendMode) -> Result<RequestToSendMode> {
        unsafe {
            let mode = libmodbus_sys::modbus_rtu_set_rts(self.ctx, mode as c_int) as u32;
            match mode {
                libmodbus_sys::MODBUS_RTU_RTS_NONE => Ok(RequestToSendMode::MODBUS_RTU_RTS_NONE),
                libmodbus_sys::MODBUS_RTU_RTS_UP => Ok(RequestToSendMode::MODBUS_RTU_RTS_UP),
                libmodbus_sys::MODBUS_RTU_RTS_DOWN => Ok(RequestToSendMode::MODBUS_RTU_RTS_DOWN),
                _ => Err("Could not set RTS mode".into()),
            }
        }
    }

    /// `rtu_get_rts` -  get the current RTS mode in RTU
    ///
    /// The [`rtu_get_rts()`](#method.rtu_get_rts) function shall get the current Request To Send mode of the libmodbus context ctx. The possible returned values are:
    ///     * MODBUS_RTU_RTS_NONE
    ///     * MODBUS_RTU_RTS_UP
    ///     * MODBUS_RTU_RTS_DOWN
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusRTU, SerialMode, RequestToSendMode};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// match modbus.rtu_set_serial_mode(SerialMode::MODBUS_RTU_RS485) {
    ///     Ok(mode) => {
    ///         assert_eq!(mode, SerialMode::MODBUS_RTU_RS485);
    ///         assert!(modbus.rtu_get_rts().is_ok())
    ///     },
    ///     Err(_) => {}
    /// };
    /// ```
    fn rtu_get_rts(&self) -> Result<RequestToSendMode> {
        unsafe {
            let mode = libmodbus_sys::modbus_rtu_get_rts(self.ctx) as u32;
            match mode {
                libmodbus_sys::MODBUS_RTU_RTS_NONE => Ok(RequestToSendMode::MODBUS_RTU_RTS_NONE),
                libmodbus_sys::MODBUS_RTU_RTS_UP => Ok(RequestToSendMode::MODBUS_RTU_RTS_UP),
                libmodbus_sys::MODBUS_RTU_RTS_DOWN => Ok(RequestToSendMode::MODBUS_RTU_RTS_DOWN),
                _ => Err("Could not get RTS mode".into()),
            }
        }
    }

    /// `rtu_set_custom_rts` - set a function to be used for custom RTS implementation
    ///
    /// The modbus_rtu_set_custom_rts() function shall set a custom function to be called when the RTS pin is to be set
    /// before and after a transmission. By default this is set to an internal function that toggles the RTS pin using an ioctl call.
    ///
    /// Note that this function adheres to the RTS mode,
    /// the values MODBUS_RTU_RTS_UP or MODBUS_RTU_RTS_DOWN must be used for the function to be called.
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// TODO: implement rtu_set_custom_rts()!
    fn rtu_set_custom_rts(&mut self, _mode: RequestToSendMode) -> Result<i32> {
        unimplemented!()
    }

    /// `rtu_get_rts_delay` - get the current RTS delay in RTU
    ///
    /// The [`rtu_get_rts_delay()`](#method.rtu_get_rts_delay) function shall get the current
    /// Request To Send  delay period of the libmodbus context ctx.
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// # Return value
    ///
    /// The [`rtu_get_rts_delay()`](#method.rtu_get_rts_delay) function shall return the current RTS delay in microseconds
    /// if successful. Otherwise it shall return `ModbusError::NotRTU`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusRTU};
    /// let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// modbus.rtu_get_rts_delay();
    /// ```
    fn rtu_get_rts_delay(&self) -> Result<i32> {
        unsafe {
            match libmodbus_sys::modbus_rtu_get_rts_delay(self.ctx) {
                -1 => Err("Could not set RTS delay".into()),
                delay => Ok(delay),
            }
        }
    }

    /// `rtu_set_rts_delay` - get the current RTS delay in RTU
    ///
    /// The [`rtu_set_rts_delay()`](#method.rtu_set_rts_delay) function shall set the Request To Send delay period of the libmodbus context.
    ///
    /// This function can only be used with a context using a RTU backend.
    ///
    /// # Return value
    ///
    /// The [`rtu_set_rts_delay()`](#method.rtu_set_rts_delay) function shall return 0 if successful.
    /// Otherwise it shall return `ModbusError::NotRTU`.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus_rs::{Modbus, ModbusRTU};
    /// let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    ///
    /// modbus.rtu_set_rts_delay(100);
    /// ```
    fn rtu_set_rts_delay(&mut self, us: i32) -> Result<i32> {
        unsafe {
            match libmodbus_sys::modbus_rtu_set_rts_delay(self.ctx, us as c_int) {
                -1 => Err("Could not set RTS delay".into()),
                _ => Ok(0),
            }
        }
    }
}
