extern crate libmodbus_rs;
#[macro_use] extern crate error_chain;

use libmodbus_rs::{Modbus, ModbusServer, ModbusTCP};
use libmodbus_rs::errors::*;


fn run() -> Result<()> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.set_debug(true)?;

    let mut socket = modbus.tcp_listen(1).unwrap();
    modbus.tcp_accept(&mut socket)?;

    Ok(())
}

fn main() {

    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
