extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};


fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();
    // 2. Parameter SlaveID
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id);
    let _ = modbus.set_debug(true);
    // let _ = modbus.rtu_set_rts(raw::MODBUS_RTU_RTS_DOWN);

    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            match modbus.write_register(3, 10) {
                Ok(_) => println!("Register successfully written."),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
