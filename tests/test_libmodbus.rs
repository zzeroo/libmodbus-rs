extern crate modbus_rs;

use modbus_rs::{Modbus, ModbusRTU};


#[test]
fn create_modbus_context() {
    assert!(Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).is_ok());
}
