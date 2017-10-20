// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate libmodbus_rs;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;
use libmodbus_rs::{Modbus, ModbusMapping, ModbusServer, ModbusTCPPI};


fn run() -> Result<()> {
    let mut modbus = Modbus::new_tcp_pi("::0", "1502").chain_err(|| "could not create modbus TCPv6 contest")?;
    let mut socket = modbus.tcp_pi_listen(1).chain_err(|| "could not listen")?;
    modbus.tcp_pi_accept(&mut socket).chain_err(|| "could not accept socket")?;

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).chain_err(|| "unable to create modbus mapping")?;
    let mut query = vec![0u8; Modbus::MAX_ADU_LENGTH as usize];

    loop {
        let request_len = modbus.receive(&mut query).chain_err(|| "could not receive")?;
        modbus.reply(&query, request_len, &modbus_mapping).chain_err(|| "could not reply")?;
    }
}

fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
