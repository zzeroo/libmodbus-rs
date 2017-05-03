#![doc(html_logo_url = "https://zzeroo.github.io/share/zzeroo-logo.png",
       html_favicon_url = "https://zzeroo.github.io/share/favicon.ico",
html_root_url = "https://zzeroo.com/")]

extern crate libc;
extern crate libmodbus_sys;

pub mod modbus;

pub use self::modbus::Modbus;
