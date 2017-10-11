extern crate libmodbus_rs;


#[test]
fn create_context() {
    use libmodbus_rs::{Modbus, ModbusRTU};
    assert!(Modbus::new_rtu("/dev/ttyS0", 115200, 'N', 8, 1).is_ok());
}

#[test]
fn get_serial_mode() {
    use libmodbus_rs::{Modbus, ModbusRTU, SerialMode};
    let modbus = Modbus::new_rtu("/dev/ttyS0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RTU_RS232);
}

#[test]
#[ignore]
fn set_serial_mode() {
    use libmodbus_rs::{Modbus, ModbusRTU, SerialMode};
    let mut modbus = Modbus::new_rtu("/dev/ttyS0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RTU_RS232);
    // assert!(modbus.rtu_set_serial_mode(SerialMode::RTU_RS485).is_ok());
    // assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RTU_RS485);
}
