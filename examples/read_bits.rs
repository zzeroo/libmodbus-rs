/// Diese Demo Funktion dient zur Abfrage eines einzelnen Bits in der Modbus Datenstruktur
/// des Modbus Slaves
/// Modbus Funktion Code:
///     - 01 (0x01) Read Coils

extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};
use std::error::Error;

fn run_command(mut modbus: Modbus, slave_id: i32, address: i32, num_bits: i32) -> Result<Vec<u8>, Box<Error>> {
    try!(modbus.set_slave(slave_id));
    try!(modbus.set_debug(true));
    try!(modbus.connect());
    let bits = try!(modbus.read_bits(address, num_bits));

    Ok(bits)
}


fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();
    // 2. Parameter SlaveID
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let address: i32 = std::env::args().nth(3).unwrap_or("0".to_string()).parse().unwrap_or(0);
    let num_bit: i32 = std::env::args().nth(4).unwrap_or("0".to_string()).parse().unwrap_or(0);

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);

    match run_command(modbus, slave_id, address, num_bit) {
        Ok(bits) => println!("Bits {:?}", bits),
        Err(err) => println!("Error: {}", err),
    }
}
