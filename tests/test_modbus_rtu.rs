use libmodbus::{Modbus, ModbusRTU, RequestToSendMode, SerialMode};


#[test]
fn new_rtu_context() {
    assert!(Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).is_ok());
}

#[test]
#[ignore]
fn rtu_get_serial_mode() {
    let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RtuRS232);
}

#[test]
#[ignore]
// FIXME: Why is serial mode RS485 not possible?
fn rtu_set_serial_mode() {
    let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RtuRS232);
    // assert!(modbus.rtu_set_serial_mode(SerialMode::RtuRS485).is_ok());
    // assert_eq!(modbus.rtu_get_serial_mode().unwrap(), SerialMode::RtuRS485);
}

#[test]
#[ignore]
fn rtu_get_rts() {
    let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_rts().unwrap(), RequestToSendMode::RtuRtsNone);
}

#[test]
#[ignore]
fn rtu_set_rts() {
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    // before
    assert_eq!(modbus.rtu_get_rts().unwrap(), RequestToSendMode::RtuRtsNone);
    // set rts mode
    assert!(modbus.rtu_set_rts(RequestToSendMode::RtuRtsUp).is_ok());
    // after set rts mode
    assert_eq!(modbus.rtu_get_rts().unwrap(), RequestToSendMode::RtuRtsUp);
}

#[test]
#[ignore]
fn rtu_set_custom_rts() {
    let _modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    // function pointer via ffi ???
}

#[test]
#[ignore]
fn rtu_get_rts_delay() {
    let modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_rts_delay().unwrap(), 86);
}

#[test]
#[ignore]
fn rtu_set_rts_delay() {
    let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).unwrap();
    assert_eq!(modbus.rtu_get_rts_delay().unwrap(), 86);
    assert!(modbus.rtu_set_rts_delay(100).is_ok());
    assert_eq!(modbus.rtu_get_rts_delay().unwrap(), 100);
}
