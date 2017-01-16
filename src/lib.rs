#![doc(html_logo_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/xmz-logo.png",
       html_favicon_url = "https://raw.githubusercontent.com/Kliemann-Service-GmbH/xMZ-Mod-Touch-Server/master/share/favicon.ico",
       html_root_url = "https://gaswarnanlagen.com/")]
extern crate libc;
extern crate libmodbus_sys as raw;
use libc::{c_int};

/// Modbus context
pub mod modbus;
/// Error handling
mod error;

// Public reexport, http://rust-lang.github.io/book/chYY-YY-public-api.html
pub use self::modbus::{Error, Modbus};
pub use self::modbus::MODBUS_RTU_RTS_DOWN;

// These republished constants are for the doc tests. Because there is no simple way to access raw:: values
pub const MODBUS_RTU_RS485: c_int = raw::MODBUS_RTU_RS485;
pub const MODBUS_RTU_RTS_DOWN: c_int = raw::MODBUS_RTU_RTS_DOWN;
