#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://zzeroo.com/")]
//! This is an 'hopefully' safe Rust interface for the [libmodbus](http://libmodbus.org/) C library from http://libmodbus.org/.
//!
//! [libmodbus](http://libmodbus.org/) is a library to send/receive data with a device which respects the Modbus protocol.
//! This library contains various backends to communicate over different networks (eg. serial in RTU mode or Ethernet in TCP/IPv6).
//! The http://www.modbus.org site provides documentation about the protocol at http://www.modbus.org/specs.php.
//!
//! libmodbus provides an abstraction of the lower communication layers and offers the same API on all supported platforms.
//!
//! This documentation presents an overview of libmodbus concepts, describes how libmodbus abstracts Modbus communication with
//! different hardware and platforms and provides a reference manual for the functions provided by the libmodbus library.
//!
//! **This project hosts the original libmodbus documentation, used here, as well.**<br />
//! Please have a look at: [libmodbus/libmodbus.html](../libmodbus/libmodbus.html)
//!
//! ## Contexts
//!
//! The Modbus protocol contains many variants (eg. serial RTU or Ehternet TCP), to ease the implementation of a variant, the library was designed to use a backend for each variant.
//!
//! **libmodbus-rs provides traits and matching implementations for these variants.** See [ModbusRTU](trait.ModbusRTU.html) (trait), [Modbus::new_rtu()](struct.Modbus.html#method.new_rtu) (implementation) or [ModbusTCP](trait.ModbusTCP.html) (trait).
//!
//! The backends are also a convenient way to fulfill other requirements (eg. real-time operations). Each backend offers a specific function to create a new modbus_t context.
//! The modbus_t context is an opaque structure containing all necessary information to establish a connection with others Modbus devices according to the selected variant.
//!
//! You can choose the best context for your needs among:
//!
//! * [RTU Context](trait.ModbusRTU.html)
//! * [TCP (IPv4) Context](trait.ModbusTCP.html)
//! * [TCP PI (IPv4 and IPv6) Context](trait.ModbusTCPPI.html)
//!
//! ### [RTU Context](trait.ModbusRTU.html)
//!
//! The RTU backend (Remote Terminal Unit) is used in serial communication and makes use of a compact, binary representation of the data for protocol communication.
//! The RTU format follows the commands/data with a cyclic redundancy check checksum as an error check mechanism to ensure the reliability of data.
//! Modbus RTU is the most common implementation available for Modbus. A Modbus RTU message must be transmitted continuously without inter-character hesitations
//! (extract from Wikipedia, Modbus, http://en.wikipedia.org/wiki/Modbus (as of Mar. 13, 2011, 20:51 GMT).
//!
//! The Modbus RTU framing calls a slave, a device/service which handle Modbus requests, and a master, a client which send requests. The communication is always initiated by the master.
//!
//! Many Modbus devices can be connected together on the same physical link so before sending a message, you must set the slave (receiver) with modbus_set_slave(3).
//! If you’re running a slave, its slave number will be used to filter received messages.
//!
//! The libmodbus implementation of RTU isn’t time based as stated in original Modbus specification,
//! instead all bytes are sent as fast as possible and a response or an indication is considered complete when all expected characters have been received.
//! This implementation offers very fast communication but you must take care to set a response timeout of slaves less than response timeout of master
//! (ortherwise other slaves may ignore master requests when one of the slave is not responding).
//!
//! * Create a Modbus RTU context
//!     - [`new_rtu()`](struct.Modbus.html#method.new_rtu)
//!
//! * Set the serial mode
//!     - [`rtu_get_serial_mode()`](struct.Modbus.html#method.rtu_get_serial_mode), [`rtu_set_serial_mode()`](struct.Modbus.html#method.rtu_set_serial_mode), [`rtu_get_rts()`](struct.Modbus.html#method.rtu_get_rts), [`rtu_set_rts()`](struct.Modbus.html#method.rtu_set_rts), [`rtu_set_custom_rts()`](struct.Modbus.html#method.rtu_set_custom_rts), [`rtu_get_rts_delay()`](struct.Modbus.html#method.rtu_get_rts_delay), [`rtu_set_rts_delay()`](struct.Modbus.html#method.rtu_set_rts_delay)
//!
//! ### [TCP (IPv4) Context](trait.ModbusTCP.html)
//! The TCP backend implements a Modbus variant used for communications over TCP/IPv4 networks.
//! It does not require a checksum calculation as lower layer takes care of the same.
//!
//! * Create a Modbus TCP context
//!     - [`new_tcp()`](struct.Modbus.html#method.new_tcp)
//!
//! ### [TCP PI (IPv4 and IPv6) Context](trait.ModbusTCPPI.html)
//! The TCP PI (Protocol Independent) backend implements a Modbus variant used for communications over TCP IPv4 and IPv6 networks.
//! It does not require a checksum calculation as lower layer takes care of the same.
//!
//! Contrary to the TCP IPv4 only backend, the TCP PI backend offers hostname resolution but it consumes about 1Kb of additional memory.
//!
//! * Create a Modbus TCP context
//!     - [`new_tcp_pi()`](struct.Modbus.html#method.new_tcp_pi)
//!
//! ### Common
//!
//! ### Connection
//!
//! ### [`Client`](trait.ModbusClient.html)
//!
//! The Modbus protocol defines different data types and functions to read and write them from/to remote devices.
//! The following functions are used by the clients to send Modbus requests:
//!
//! * Read data
//!     - [`read_bits()`](struct.Modbus.html#method.read_bits), [`read_input_bits()`](struct.Modbus.html#method.read_input_bits), [`read_registers()`](struct.Modbus.html#method.read_registers), [`read_input_registers()`](struct.Modbus.html#method.read_input_registers), [`report_slave_id()`](struct.Modbus.html#method.report_slave_id)
//! * Write data
//!     - [`write_bit()`](struct.Modbus.html#method.write_bit), [`write_register()`](struct.Modbus.html#method.write_register), [`write_bits()`](struct.Modbus.html#method.write_bits), [`write_registers()`](struct.Modbus.html#method.write_registers)
//! * Write and read data
//!     - [`write_and_read_registers()`](struct.Modbus.html#method.write_and_read_registers)
//! * Raw requests
//!     - [`send_raw_request()`](struct.Modbus.html#method.send_raw_request), [`receive_confirmation()`](struct.Modbus.html#method.receive_confirmation)
//! * Reply an exception
//!     - [`reply_exception()`](struct.Modbus.html#method.reply_exception)
//!
//! ### [`Server`](trait.ModbusServer.html)
//!
//! The server is waiting for request from clients and must answer when it is concerned by the request.
//!
//! In TCP , you must not use the usual [`connect()`](struct.Modbus.html#method.connect) to establish the connection but a pair of accept/listen calls
//!
//! * [`tcp_listen()`](struct.Modbus.html#method.tcp_listen), [`tcp_accept()`](struct.Modbus.html#method.tcp_accept), [`tcp_pi_listen`()](struct.Modbus.html#method.tcp_pi_listen), [`tcp_pi_accept`()](struct.Modbus.html#method.tcp_pi_accept)
//!
//! then the data can be received with
//!
//! * [`receive()`](struct.Modbus.html#method.receive)
//!
//! and a response can be send with
//!
//! * [`reply()`](struct.Modbus.html#method.reply), [`reply_exception()`](struct.Modbus.html#method.reply_exception)
//!
//! To handle the mapping of your Modbus data, you must use a [`ModbusMapping`](struct.ModbusMapping.html) struct: [`ModbusMapping::new()`](struct.ModbusMapping.html#method.new)
//!

// `error_chain!` can recurse deeply(3)
#![recursion_limit = "1024"]

extern crate libc;
extern crate libmodbus_sys;
#[macro_use] extern crate error_chain;

pub mod errors;
mod modbus_client;
mod modbus_mapping;
mod modbus_rtu;
mod modbus_server;
mod modbus_tcp_pi;
mod modbus_tcp;
mod modbus;

pub use self::modbus_client::ModbusClient;
pub use self::modbus_mapping::ModbusMapping;
pub use self::modbus_rtu::{ModbusRTU, RequestToSendMode, SerialMode};
pub use self::modbus_server::ModbusServer;
pub use self::modbus_tcp_pi::ModbusTCPPI;
pub use self::modbus_tcp::ModbusTCP;
pub use self::modbus::Modbus;

pub use libmodbus_sys::MODBUS_TCP_DEFAULT_PORT;
pub use libmodbus_sys::MODBUS_TCP_MAX_ADU_LENGTH;
pub use libmodbus_sys::MODBUS_MAX_ADU_LENGTH;
