extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};


fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();
    // 2. Parameter SlaveID
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();
    let address: i32 = std::env::args().nth(3).unwrap_or("0".to_string()).parse().unwrap();
    let _values = std::env::args().nth(4).unwrap_or("".to_string()); // Einzelne Schritte, so das der Borrowchecker ruhig ist
    let _values: Vec<&str> = _values.split(",").collect(); // Einzelne Schritte, so das der Borrowchecker ruhig ist
    let values: Vec<u16> = _values.into_iter().map(|x| x.parse::<u16>().unwrap_or(0)).collect();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id);
    let _ = modbus.set_debug(true);

    // println!("device: {}, slave_id: {}, address: {}, values: {:?}", device, slave_id, address, values);

    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            match modbus.write_registers(address, &values) {
                Ok(num) => println!("{} registers successfully written.", num),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
