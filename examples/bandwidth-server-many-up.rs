extern crate libmodbus_rs;

mod unit_test_config;

use libmodbus_rs::{Modbus, ModbusMapping, ModbusServer, ModbusTCP, ModbusRTU};
use std::env;
use std::io::Error;
use unit_test_config::*;


const NB_CONNECTION: i32 = 5;

fn run() -> Result<(), std::io::Error> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("Could not create modbus TCP context");

    let mut mb_mapping = ModbusMapping::new(Modbus::MAX_READ_BITS, 0,
                                            Modbus::MAX_READ_REGISTERS, 0).expect("Failed to allocate the mapping");

    let mut server_socket = modbus.tcp_listen(NB_CONNECTION).expect("Unable to listen TCP connection");


    Ok(())
}

fn main() {
    if let Err(ref err) = run() {
        println!("{}", Modbus::strerror(err.raw_os_error().unwrap()));

        std::process::exit(1);
    }
}
