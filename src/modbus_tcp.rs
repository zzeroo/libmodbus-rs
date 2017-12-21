use failure::Error;
use libmodbus_sys as ffi;
use modbus::Modbus;
use std::ffi::CString;


/// The TCP backend implements a Modbus variant used for communications over TCP/IPv4 networks.
/// It does not require a checksum calculation as lower layer takes care of the same.
///
/// * Create a Modbus TCP context
///     - [`new_tcp()`](struct.Modbus.html#method.new_tcp)
///
pub trait ModbusTCP {
    fn new_tcp(ip: &str, port: i32) -> Result<Modbus, Error>;
    fn tcp_accept(&mut self, socket: &mut i32) -> Result<i32, Error>;
    fn tcp_listen(&mut self, num_connection: i32) -> Result<i32, Error>;
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
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let modbus = Modbus::new_tcp("127.0.0.1", Modbus::TCP_DEFAULT_PORT as i32).unwrap();
    ///
    /// match modbus.connect() {
    ///     Ok(_) => {  }
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    fn new_tcp(ip: &str, port: i32) -> Result<Modbus, Error> {
        unsafe {
            let ip = CString::new(ip).unwrap();
            let ctx = ffi::modbus_new_tcp(ip.as_ptr(), port);

            if ctx.is_null() {
                bail!(::std::io::Error::last_os_error())
            } else {
                Ok(Modbus { ctx: ctx })
            }
        }
    }

    /// `tcp_accept` - accept a new connection on a TCP Modbus socket (IPv4)
    ///
    /// The [`tcp_accept()`](#method.tcp_accept) function shall extract the first connection on the
    /// queue of pending connections and create a new socket given as argument.
    ///
    /// # Parameters
    ///
    /// * `socket`  - Socket
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    /// let mut socket = modbus.tcp_listen(1).unwrap();
    ///
    /// modbus.tcp_accept(&mut socket);
    /// ```
    fn tcp_accept(&mut self, socket: &mut i32) -> Result<i32, Error> {
        unsafe {
            match ffi::modbus_tcp_accept(self.ctx, socket) {
                -1 => bail!(::std::io::Error::last_os_error()),
                socket => Ok(socket),
            }
        }
    }


    /// `tcp_listen` - create and listen a TCP Modbus socket (IPv4)
    ///
    /// The [`tcp_listen()`](#method.tcp_listen) function shall create a socket and listen to maximum
    /// `num_connection` incoming connections on the specified IP address.
    /// If IP address is set to NULL or '0.0.0.0', any addresses will be listen.
    ///
    /// # Parameters
    ///
    /// * `num_connection`  - maximum number of incoming connections on the specified IP address
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use libmodbus_rs::{Modbus, ModbusTCP};
    ///
    /// let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    ///
    /// let socket = modbus.tcp_listen(1);
    /// ```
    fn tcp_listen(&mut self, num_connection: i32) -> Result<i32, Error> {
        unsafe {
            match ffi::modbus_tcp_listen(self.ctx, num_connection) {
                -1 => bail!(::std::io::Error::last_os_error()),
                socket => Ok(socket),
            }
        }
    }
}
