//! The TCP PI (Protocol Independent) backend implements a Modbus variant used for communications over TCP IPv4 and IPv6 networks.
//! It does not require a checksum calculation as lower layer takes care of the same.
//!
//! Contrary to the TCP IPv4 only backend, the TCP PI backend offers hostname resolution but it consumes about 1Kb of additional memory.
//!
//! Create a Modbus TCP context
//!
//! modbus_new_tcp_pi(3)
//!
use libmodbus_sys;
use modbus::Modbus;
use std::ffi::CString;
use std::io::Error;

pub trait ModbusTCPPI {
    fn new_tcp_pi(node: &str, service: &str) -> Result<Modbus, Error>;
}

impl ModbusTCPPI for Modbus {
    /// `new_tcp_pi` - create a libmodbus context for TCP Protocol Independent
    ///
    /// The [`new_tcp_pi()`](#method.new_tcp_pi) function shall allocate and initialize a modbus_t structure to communicate with a Modbus TCP IPv4 or IPv6 server.
    ///
    /// The **node** argument specifies the host name or IP address of the host to connect to, eg. "192.168.0.5" , "::1" or "server.com".
    /// A NULL value can be used to listen any addresses in server mode.
    ///
    /// The **service** argument is the service name/port number to connect to. To use the default Modbus port use the string "502".
    /// On many Unix systems, it’s convenient to use a port number greater than or equal to 1024 because it’s not necessary to have administrator privileges.
    ///
    /// # Examples
    ///
    /// ```
    /// use modbus_rs::{Modbus, ModbusTCPPI};
    ///
    /// let modbus = Modbus::new_tcp_pi("::1", "1502").unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {}
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    fn new_tcp_pi(node: &str, service: &str) -> Result<Modbus, Error> {
        unsafe {
            let node = CString::new(node).unwrap();
            let service = CString::new(service).unwrap();
            let ctx = libmodbus_sys::modbus_new_tcp_pi(node.as_ptr(),
                service.as_ptr());

            if ctx.is_null() {
                Err(Error::last_os_error())
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }
}
