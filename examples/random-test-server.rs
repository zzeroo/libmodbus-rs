extern crate libmodbus_rs;
#[macro_use] extern crate error_chain;

use libmodbus_rs::{Modbus, ModbusServer, ModbusTCP, MODBUS_TCP_MAX_ADU_LENGTH};
use libmodbus_rs::errors::*;


fn run() -> Result<()> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let mut socket = modbus.tcp_listen(1).unwrap();
    modbus.set_debug(true)?;

    modbus.tcp_accept(&mut socket);

    loop {
        let mut query = vec![0u8; MODBUS_TCP_MAX_ADU_LENGTH as usize];

        println!("{:?}", modbus.receive(&mut query) );
        break;
    }

    Ok(())
}

fn main() {

    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
