#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://zzeroo.com/")]
//! libmodbus is a library to send/receive data with a device which respects the Modbus protocol.
//! This library contains various backends to communicate over different networks (eg. serial in RTU mode or Ethernet in TCP/IPv6).
//! The http://www.modbus.org site provides documentation about the protocol at http://www.modbus.org/specs.php.
//!
//! libmodbus provides an abstraction of the lower communication layers and offers the same API on all supported platforms.
//!
//! This documentation presents an overview of libmodbus concepts, describes how libmodbus abstracts Modbus communication with
//! different hardware and platforms and provides a reference manual for the functions provided by the libmodbus library.
//!
//! ## Contexts
//!
//! The Modbus protocol contains many variants (eg. serial RTU or Ehternet TCP), to ease the implementation of a variant,
//! the library was designed to use a backend for each variant.
//! The backends are also a convenient way to fulfill other requirements (eg. real-time operations). Each backend offers a specific function to create a new modbus_t context.
//! The modbus_t context is an opaque structure containing all necessary information to establish a connection with others Modbus devices according to the selected variant.
//!
//! You can choose the best context for your needs among:
//!
//! * [RTU Context](trait.ModbusRTU.html)
//! * [TCP (IPv4) Context](trait.ModbusTCP.html)
//! * [TCP PI (IPv4 and IPv6) Context](trait.ModbusTCPPI.html)
//!

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

extern crate libc;
extern crate libmodbus_sys;
#[macro_use] extern crate error_chain;

mod error;
mod modbus_rtu;
mod modbus_tcp;
mod modbus_tcp_pi;
mod modbus;
mod modbus_client;
mod modbus_server;
mod errors;

pub use self::modbus_rtu::{ModbusRTU, RTUMode, SerialMode};
pub use self::modbus_tcp::{ModbusTCP};
pub use self::modbus_tcp_pi::{ModbusTCPPI};
pub use self::modbus::Modbus;
pub use self::modbus_client::ModbusClient;
pub use self::modbus_server::ModbusServer;


pub use libmodbus_sys::MODBUS_TCP_DEFAULT_PORT;
