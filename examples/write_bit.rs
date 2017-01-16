extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};
use std::error::Error;


fn run_command(mut modbus: Modbus, slave_id: i32, address: i32, state: bool) -> Result<(), Box<Error>> {
    try!(modbus.set_slave(slave_id));
    try!(modbus.set_debug(true));

    try!(modbus.connect());
    try!(modbus.write_bit(address, state));

    Ok(())
}

fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();
    // 2. Parameter SlaveID
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let address = std::env::args().nth(3).unwrap().parse().unwrap_or(0);
    let state: bool = std::env::args().nth(4).unwrap().parse().unwrap_or(false);

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id);
    let _ = modbus.set_debug(true);

    match run_command(modbus, slave_id, address, state) {
        Ok(_) => {}
        Err(err) => println!("Error: {}", err),
    }
}
