#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate libmodbus_rs;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}
mod unit_test_config;

use unit_test_config::*;
use errors::*;
use libmodbus_rs::{Modbus, ModbusMapping, ModbusServer, ModbusTCP, ModbusTCPPI, ModbusRTU};
use std::env;

const EXCEPTION_RC: u32 = 2;


fn run() -> Result<()> {
    let backend;
    let mut query;
    let modbus: Modbus;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].to_lowercase().as_ref() {
            "tcp" => backend = Backend::TCP,
            "tcppi" => backend = Backend::TCPPI,
            "rtu" => backend = Backend::RTU,
            _ => {
                println!("Usage:\n  {} [tcp|tcppi|rtu] - Modbus server for unit testing\n\n", args[0]);
                std::process::exit(-1);
            },
        }
    } else {
        // By default
        backend = Backend::TCP;
    }

    // Setup modbus context
    let mut modbus = match backend {
            Backend::TCP => {
                query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
                Modbus::new_tcp("127.0.0.1", 1502)
            },
            Backend::TCPPI => {
                query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
                Modbus::new_tcp_pi("::0", "1502")
            },
            Backend::RTU => {
                query = vec![0u8; Modbus::RTU_MAX_ADU_LENGTH as usize];
                Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1)
            },
        }.chain_err(|| "could not select backend")?;

    if backend == Backend::RTU {
        modbus.set_slave(SERVER_ID).chain_err(|| format!("could not set modbus slave address {}", SERVER_ID))?;
    }

    let header_length = modbus.get_header_length();

    modbus.set_debug(true).expect("could not set modbus DEBUG mode");


    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
