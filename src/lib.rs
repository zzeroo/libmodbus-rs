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
pub use self::raw::{MODBUS_RTU_RTS_DOWN, MODBUS_RTU_RS485};
