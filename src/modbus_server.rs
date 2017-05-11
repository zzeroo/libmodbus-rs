use errors::*;
use libc::{c_char, c_int};
use libmodbus_sys;
use modbus::Modbus;


/// The server is waiting for request from clients and must answer when it is concerned by the request. The libmodbus offers the following functions to handle requests:
///
/// * Receive
///     - [`receive()`](struct.Modbus.html#method.receive)
/// * Reply
///     - [`reply()`](struct.Modbus.html#method.reply), [`reply_exception()`](struct.Modbus.html#method.reply_exception)
///
pub trait ModbusServer {
    fn receive(&self, request: &mut [u8]) -> Result<i32>;
}

impl ModbusServer for Modbus {
    /// `receive` - receive an indication request
    ///
    /// The [`receive()`](#method.receive) function shall receive an indication request from the socket of the context ctx.
    /// This function is used by Modbus slave/server to receive and analyze indication request sent by the masters/clients.
    ///
    /// If you need to use another socket or file descriptor than the one defined in the context ctx, see the function [`set_socket()`](struct.Modbus.html#method.set_socket).
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusServer, ModbusTCP, MODBUS_MAX_ADU_LENGTH};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut query = vec![0; MODBUS_MAX_ADU_LENGTH as usize];
    ///
    /// assert!(modbus.receive(&mut query).is_ok());
    /// ```
    fn receive(&self, request: &mut [u8]) -> Result<i32> {
        assert!(request.len() <= libmodbus_sys::MODBUS_TCP_MAX_ADU_LENGTH as usize);
        unsafe {
            let len = libmodbus_sys::modbus_receive(self.ctx, request.as_mut_ptr() );
            match len {
                -1 => Err("Could not receive an idication request".into()),
                len => Ok(len),
            }
        }
    }
}
