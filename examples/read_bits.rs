/// Diese Demo Funktion dient zur Abfrage eines einzelnen Bits in der Modbus Datenstruktur
/// des Modbus Slaves
/// Modbus Funktion Code:
///     - 01 (0x01) Read Coils

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
    let address = std::env::args().nth(3).unwrap().parse().unwrap_or(0);
    let num_bit = std::env::args().nth(4).unwrap().parse().unwrap_or(1);

    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            match modbus.read_bits(address, num_bit) {
                Ok(bits) => println!("Bits {:?}", bits),
                Err(err) => println!("Error: {}", err),
            }
        }
    }
}
