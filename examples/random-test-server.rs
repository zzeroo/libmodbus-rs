extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer, ModbusMapping, ModbusTCP, MODBUS_TCP_MAX_ADU_LENGTH};
use libmodbus_rs::errors::*;


fn run() -> Result<()> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;
    modbus.set_debug(true)?;

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500)?;

    let mut socket = modbus.tcp_listen(1)?;
    modbus.tcp_accept(&mut socket)?;


    loop {
        let mut query = vec![0u8; MODBUS_TCP_MAX_ADU_LENGTH as usize];

        match modbus.receive(&mut query) {
            Ok(n) => {
                modbus.reply(&query, n, &modbus_mapping)
            }
            Err(_) => {
                break
            }
        }?;
    }
    Err("Quit the loop. This is the ok, default behavior.".into())

    // ModbusMapping and Modbus struct are dropped here, automatically by Rusts Drop trait.
    // Hence no manual `modbus_mapping_free(mb_mapping)`, `modbus_close(ctx)` nor `modbus_free(ctx)` needed, like in C.
}

fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
