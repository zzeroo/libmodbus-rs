// This test is not part of the original libmodbus lib!
//
// It shows how to use the ModbusTCPPI context.
//
extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusMapping, ModbusServer, ModbusTCP, MODBUS_MAX_ADU_LENGTH};
use libmodbus_rs::errors::*; // for the `Result<T>` type


fn run() -> Result<()> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;
    let mut socket = modbus.tcp_listen(1)?;
    modbus.tcp_accept(&mut socket)?;

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500)?;
    let mut query = vec![0u8; MODBUS_MAX_ADU_LENGTH as usize];

    loop {
        let request_len = modbus.receive(&mut query)?;
        modbus.reply(&query, request_len, &modbus_mapping)?;
    }
}

fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
