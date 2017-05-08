extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer, ModbusTCP, MODBUS_MAX_ADU_LENGTH};


fn main() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let mut query = vec![0; MODBUS_MAX_ADU_LENGTH as usize];
    assert!(modbus.receive(&mut query).is_ok());
}
