extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};


fn main() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.set_debug(true);

    match modbus.connect() {
        Ok(_) => {  }
        Err(e) => println!("Error: {}", e),
    }
}
