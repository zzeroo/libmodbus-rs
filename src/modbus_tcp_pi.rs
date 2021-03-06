use crate::prelude::*;
use libmodbus_sys as ffi;
use std::ffi::CString;

/// The TCP PI (Protocol Independent) backend implements a Modbus variant used for communications over TCP IPv4 and
/// IPv6 networks.
/// It does not require a checksum calculation as lower layer takes care of the same.
///
/// Contrary to the TCP IPv4 only backend, the TCP PI backend offers hostname resolution but it consumes about 1Kb of
/// additional memory.
///
/// * Create a Modbus TCP context
///     - [`new_tcp_pi()`](struct.Modbus.html#method.new_tcp_pi)
///
pub trait ModbusTCPPI {
    fn new_tcp_pi(node: &str, service: &str) -> Result<Modbus, Error>;
    fn tcp_pi_accept(&mut self, socket: &mut i32) -> Result<i32, Error>;
    fn tcp_pi_listen(&mut self, num_connection: i32) -> Result<i32, Error>;
}

impl ModbusTCPPI for Modbus {
    /// `new_tcp_pi` - create a libmodbus context for TCP Protocol Independent
    ///
    /// The [`new_tcp_pi()`](#method.new_tcp_pi) function shall allocate and initialize a modbus_t structure to
    /// communicate with a Modbus TCP IPv4 or IPv6 server.
    ///
    /// The **node** argument specifies the host name or IP address of the host to connect to, eg. "192.168.0.5" ,
    /// "::1" or "server.com".
    /// A NULL value can be used to listen any addresses in server mode.
    ///
    /// The **service** argument is the service name/port number to connect to. To use the default Modbus port use the
    /// string "502".
    /// On many Unix systems, it’s convenient to use a port number greater than or equal to 1024 because it’s not
    /// necessary to have administrator privileges.
    ///
    /// # Examples
    ///
    /// ```
    /// use libmodbus::{Modbus, ModbusTCPPI};
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
            let ctx = ffi::modbus_new_tcp_pi(node.as_ptr(), service.as_ptr());

            if ctx.is_null() {
                Err(Error::TcpPi {
                    msg: "new_tcp_pi".to_owned(),
                    source: ::std::io::Error::last_os_error(),
                })
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }

    /// `tcp_pi_accept` - accept a new connection on a TCP PI Modbus socket (IPv6)
    ///
    /// The [`tcp_pi_accept()`](#method.tcp_pi_accept) function shall extract the first connection on the
    /// queue of pending connections and create a new socket given as argument.
    ///
    /// # Parameters
    ///
    /// * `socket`  - Socket
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus::{Modbus, ModbusMapping, ModbusServer, ModbusTCPPI};
    ///
    /// let mut modbus = Modbus::new_tcp_pi("::0", "1502").unwrap();
    /// let mut socket = modbus.tcp_pi_listen(1).unwrap();
    /// modbus.tcp_pi_accept(&mut socket).unwrap();
    /// ```
    fn tcp_pi_accept(&mut self, socket: &mut i32) -> Result<i32, Error> {
        unsafe {
            match ffi::modbus_tcp_pi_accept(self.ctx, socket) {
                -1 => Err(Error::TcpPi {
                    msg: "tcp_pi_accept".to_owned(),
                    source: ::std::io::Error::last_os_error(),
                }),
                socket => Ok(socket),
            }
        }
    }

    /// `tcp_pi_listen` - create and listen a TCP PI Modbus socket (IPv6)
    ///
    /// The [`tcp_pi_listen()`](#method.tcp_pi_listen) function shall create a socket and listen to maximum
    /// `num_connection` incoming connections on the specifieded node.
    ///
    /// # Parameters
    ///
    /// * `num_connection`  - maximum number of incoming connections on the specified IP address
    ///
    /// If node is set to `""` or `0.0.0.0`, any addresses will be listen.
    ///
    /// # Examples
    ///
    /// For detailed examples, look at the examples directory of this crate.
    ///
    /// * unit-test-server.rs   - simple but handle only one connection
    /// * bandwidth-server-many-up.rs   - handles several connection at once
    ///
    /// ```rust,no_run
    /// use libmodbus::{Modbus, ModbusMapping, ModbusServer, ModbusTCPPI};
    ///
    /// let mut modbus = Modbus::new_tcp_pi("::0", "1502").unwrap();
    /// let mut socket = modbus.tcp_pi_listen(1).unwrap();
    ///
    /// modbus.tcp_pi_accept(&mut socket);
    ///
    /// let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();
    /// let mut query = vec![0u8; Modbus::MAX_ADU_LENGTH as usize];
    ///
    /// loop {
    ///     let request_len = modbus.receive(&mut query).unwrap();
    ///     modbus.reply(&query, request_len, &modbus_mapping);
    /// }
    /// ```
    fn tcp_pi_listen(&mut self, num_connection: i32) -> Result<i32, Error> {
        unsafe {
            match ffi::modbus_tcp_pi_listen(self.ctx, num_connection) {
                -1 => Err(Error::TcpPi {
                    msg: "tcp_pi_listen".to_owned(),
                    source: ::std::io::Error::last_os_error(),
                }),
                socket => Ok(socket),
            }
        }
    }
}
