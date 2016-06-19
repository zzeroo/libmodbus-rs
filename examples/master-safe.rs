extern crate libmodbus_sys as raw;
extern crate libmodbus_rs;

use libmodbus_rs::modbus::{Modbus};


fn main() {
    let device: String = std::env::args().nth(1).unwrap();
    let slave_id: i32 = std::env::args().nth(2).unwrap().parse().unwrap();

    let mut modbus = Modbus::new_rtu(&device, 9600, 'N', 8, 1);
    let _ = modbus.set_slave(slave_id);
    let _ = modbus.set_debug(true);
    let _ = modbus.rtu_set_rts(raw::MODBUS_RTU_RTS_DOWN);

    match modbus.connect() {
        Err(_) => { modbus.free(); }
        Ok(_) => {
            let tab_reg = modbus.read_registers(0, 20);
            for i in 0..20 {
                println!("register[{}]=[{}] (0x{:X})", i, &tab_reg[i as usize], &tab_reg[i as usize]);
            }
        }
    }
}
