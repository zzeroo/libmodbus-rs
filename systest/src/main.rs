extern crate libc;

use libc::*;
use libmodbus_sys::*;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
