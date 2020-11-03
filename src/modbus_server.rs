use failure::Error;
use libmodbus_sys as ffi;
use crate::prelude::*;


/// The server is waiting for request from clients and must answer when it is concerned by the request. The libmodbus
/// offers the following functions to handle requests:
///
/// * Receive
///     - [`receive()`](struct.Modbus.html#method.receive)
/// * Reply
///     - [`reply()`](struct.Modbus.html#method.reply), [`reply_exception()`](struct.Modbus.html#method.reply_exception)
///
pub trait ModbusServer {
    fn receive(&self, request: &mut [u8]) -> Result<i32, Error>;
    fn reply(&self, request: &[u8], request_len: i32, modbus_mapping: &ModbusMapping) -> Result<i32, Error>;
}

impl ModbusServer for Modbus {
    /// `receive` - receive an indication request
    ///
    /// The [`receive()`](#method.receive) function shall receive an indication request from the socket of the context
    /// ctx.
    /// This function is used by Modbus slave/server to receive and analyze indication request sent by the
    /// masters/clients.
    ///
    /// If you need to use another socket or file descriptor than the one defined in the context ctx, see the function
    /// [`set_socket()`](struct.Modbus.html#method.set_socket).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus::{Modbus, ModbusServer, ModbusTCP};
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut query = vec![0; Modbus::MAX_ADU_LENGTH as usize];
    ///
    /// assert!(modbus.receive(&mut query).is_ok());
    /// ```
    fn receive(&self, request: &mut [u8]) -> Result<i32, Error> {
        assert!(request.len() <= Modbus::MAX_ADU_LENGTH as usize);

        unsafe {
            let len = ffi::modbus_receive(self.ctx, request.as_mut_ptr());
            match len {
                -1 => bail!(::std::io::Error::last_os_error()),
                len => Ok(len),
            }
        }
    }

    /// `modbus_reply` - send a reponse to the received request
    ///
    /// The [`reply()`](#method.reply) function shall send a response to received request. The request req given in
    /// argument is analyzed, a response is then built and sent by using the information of the modbus context ctx.
    /// If the request indicates to read or write a value the operation will done in the modbus mapping mb_mapping
    /// according to the type of the manipulated data.
    /// If an error occurs, an exception response will be sent.
    ///
    /// This function is designed for Modbus server.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus::{Modbus, ModbusServer, ModbusTCP};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut query = vec![0; Modbus::MAX_ADU_LENGTH as usize];
    ///
    /// assert!(modbus.receive(&mut query).is_ok());
    /// ```
    fn reply(&self, request: &[u8], request_len: i32, modbus_mapping: &ModbusMapping) -> Result<i32, Error> {
        unsafe {
            let len =
                ffi::modbus_reply(self.ctx, request.as_ptr(), request_len, modbus_mapping.modbus_mapping);
            match len {
                -1 => bail!(::std::io::Error::last_os_error()),
                len => Ok(len),
            }
        }
    }
}
