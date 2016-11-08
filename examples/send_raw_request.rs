extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};


fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();
    // 2. Parameter SlaveID
    let slave_id: u8 = std::env::args().nth(2).unwrap().parse().unwrap();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id as i32);
    let _ = modbus.set_debug(true);
    let raw_request = vec![slave_id, 0x08, 0x0, 0x01];

    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            match modbus.send_raw_request(&raw_request) {
                Ok(_) => {},
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
