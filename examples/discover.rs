extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};


fn main() {
    // 1. Parameter Serial Interface `/dev/ttyUSB0`
    let device: String = std::env::args().nth(1).unwrap();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);

    println!("Starte suche");
    for i in (1..247).rev() {
        let _ = modbus.set_slave(i);
        // let _ = modbus.set_debug(true);
        // let _ = modbus.rtu_set_rts(raw::MODBUS_RTU_RTS_DOWN);

        match modbus.connect() {
            Err(_) => {
                modbus.free();
            }
            Ok(_) => {
                match modbus.read_registers(0, 1) {
                    Ok(_) => {
                        println!("\nSlave mit der ID: {} gefunden!", i);
                        modbus.free();
                        break;
                    }
                    Err(_) => { modbus.free(); }
                }
            }
        }
    }
}
