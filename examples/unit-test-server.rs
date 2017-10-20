#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
// `error_chain!` can recurse deeply
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

    let modbus_mapping =
        ModbusMapping::new_start_address(BITS_ADDRESS,
                                         BITS_NB,
                                         INPUT_BITS_ADDRESS,
                                         INPUT_BITS_NB,
                                         REGISTERS_ADDRESS,
                                         REGISTERS_NB,
                                         INPUT_REGISTERS_ADDRESS,
                                         INPUT_REGISTERS_NB).chain_err(|| "Failed to allocate the mapping")?;

    // Examples from PI_MODBUS_300.pdf.
    // Only the read-only input values are assigned.

    // Initialize input values that's can be only done server side.
    libmodbus_rs::set_bits_from_bytes(modbus_mapping.get_input_bits_mut(), 0, INPUT_BITS_NB, &INPUT_BITS_TAB);

    //  Initialize values of INPUT REGISTERS
    for i in 0..INPUT_REGISTERS_NB {
        modbus_mapping.get_input_registers_mut()[i as usize] = INPUT_REGISTERS_TAB[i as usize];
    }

    match backend {
        Backend::TCP => {
            let mut socket = modbus.tcp_listen(1).chain_err(|| "could not listen to TCP socket")?;
            modbus.tcp_accept(&mut socket).chain_err(|| "could not accept socket")?;
        },
        Backend::TCPPI => {
            let mut socket = modbus.tcp_pi_listen(1).chain_err(|| "could not listen to TCPv6 socket")?;
            modbus.tcp_pi_accept(&mut socket).chain_err(|| "could not accept socket")?;
        },
        Backend::RTU => {
            modbus.connect().chain_err(|| "Unable to connect")?;
        },
    }

    loop {
        match modbus.receive(&mut query) {
            Err(_err) => break,
            Ok(_) => {
                if query[header_length as usize] == 0x03 {
                    // Read holding registers
                    println!("read holding registers");
                }
            },
        }
    }

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
