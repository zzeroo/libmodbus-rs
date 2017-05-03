#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://zzeroo.com/")]

extern crate libc;
extern crate libmodbus_sys;

pub mod modbus;
pub mod error;

pub use self::modbus::{Modbus, SerialMode, RTUMode};

pub use libmodbus_sys::MODBUS_TCP_DEFAULT_PORT;
