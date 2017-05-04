//! The TCP backend implements a Modbus variant used for communications over TCP/IPv4 networks.
//! It does not require a checksum calculation as lower layer takes care of the same.
//!
//! Create a Modbus TCP context
//!
//! [`new_tcp()`](trait.ModbusTCP.html#method.new_tcp)
//!
use libmodbus_sys;
use modbus::Modbus;
use libc::c_int;
use std::ffi::CString;
use std::io::Error;


pub trait ModbusTCP {
    fn new_tcp(ip: &str, port: u32) -> Result<Modbus, Error>;
}

impl ModbusTCP for Modbus {
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
    /// use modbus_rs::{Modbus, ModbusTCP, MODBUS_TCP_DEFAULT_PORT};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus = Modbus::new_tcp("127.0.0.1", MODBUS_TCP_DEFAULT_PORT).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    fn new_tcp(ip: &str,
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
}
