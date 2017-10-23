#![allow(unused_imports)]
extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP};


#[test]
fn new() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(ModbusMapping::new(500, 500, 500, 500).is_ok());
}

#[test]
fn new_start_address() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(ModbusMapping::new_start_address(0, 0, 0, 0, 10000, 10, 0, 0).is_ok());
}

#[test]
#[ignore]
fn free() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.free();
}

#[test]
fn get_bits() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_bits(), [0u8, 0, 0, 0, 0]);
}

#[test]
fn get_bits_mut() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_bits_mut(), [0u8, 0, 0, 0, 0]);
}

#[test]
fn get_input_bit() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_bits(), [0u8, 0, 0, 0, 0])
}

#[test]
fn get_input_bits_mut() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_bits_mut(), [0u8, 0, 0, 0, 0])
}

#[test]
fn get_input_registers() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_registers(), [0u16, 0, 0, 0, 0])
}

#[test]
fn get_input_registers_mut() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_registers_mut(), [0u16, 0, 0, 0, 0])
}

#[test]
fn get_registers() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_registers(), [0u16, 0, 0, 0, 0])
}

#[test]
fn get_registers_mut() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let modbus_mapping = ModbusMapping::new(5, 5, 5, 5).unwrap();

    assert_eq!(modbus_mapping.get_input_registers_mut(), [0u16, 0, 0, 0, 0])
}
