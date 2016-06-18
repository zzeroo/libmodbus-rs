extern crate libc;
extern crate libmodbus_rs;
use libc::{c_int};
use libmodbus_rs::modbus::*;

/// The goal of this program is to check all major functions of libmodbus:
/// - write_coil
/// - read_bits
/// - write_coils
/// - write_register
/// - read_register
/// - write_registers
/// - read_registers
///
/// All these functions are called with random values on a address range defined by the
/// following constants
const LOOP: c_int                 = 1;
const SERVER_ID: c_int            = 17;
const ADDRESS_START: c_int        = 0;
const ADDRESS_END: c_int          = 99;



fn main() {
    // let mut ctx: modbus::modbus_t = Default::default();
    // ctx.set_slave(SERVER_ID);
}
