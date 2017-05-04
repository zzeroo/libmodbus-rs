#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://zzeroo.com/")]

extern crate libc;
extern crate libmodbus_sys;

pub mod error;
pub mod modbus_rtu;
pub mod modbus_tcp;
pub mod modbus;

pub use self::modbus_rtu::{ModbusRTU, RTUMode, SerialMode};
pub use self::modbus_tcp::{ModbusTCP};
pub use self::modbus::Modbus;

pub use libmodbus_sys::MODBUS_TCP_DEFAULT_PORT;
