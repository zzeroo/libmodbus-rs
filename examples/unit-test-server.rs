#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]
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
use libmodbus_rs::{Modbus, ModbusMapping, ModbusTCP, ModbusTCPPI, ModbusRTU};
use std::env;


mod unit_test_config {
    pub const SERVER_ID: u8 = 17;
    pub const INVALID_SERVER_ID: i32 = 18;

    pub const BITS_ADDRESS: u16 = 0x130;
    pub const BITS_NB: u16 = 0x25;
    pub const BITS_TAB: &[u8] = &[0xCD, 0x6B, 0xB2, 0x0E, 0x1B];

    pub const INPUT_BITS_ADDRESS: u16 = 0x1C4;
    pub const INPUT_BITS_NB: u16 = 0x16;
    pub const INPUT_BITS_TAB: &[u8] = &[0xAC, 0xDB, 0x35];

    pub const REGISTERS_ADDRESS: u16 = 0x160;
    pub const REGISTERS_NB: u16 = 0x3;
    pub const REGISTERS_NB_MAX: u16 = 0x20;
    pub const REGISTERS_TAB: &[u16] = &[0x022B, 0x0001, 0x0064];

    // Raise a manual exception when this address is used for the first byte
    pub const REGISTERS_ADDRESS_SPECIAL: u16 = 0x170;
    // The response of the server will contains an invalid TID or slave
    pub const REGISTERS_ADDRESS_INVALID_TID_OR_SLAVE: u16 = 0x171;
    // The server will wait for 1 second before replying to test timeout
    pub const REGISTERS_ADDRESS_SLEEP_500_MS: u16 = 0x172;
    // The server will wait for 5 ms before sending each byte
    pub const REGISTERS_ADDRESS_BYTE_SLEEP_5_MS: u16 = 0x173;

    // If the following value is used, a bad response is sent.
    // It's better to test with a lower value than
    // REGISTERS_NB_POINTS to try to raise a segfault.
    pub const REGISTERS_NB_SPECIAL: u16 = 0x2;

    pub const INPUT_REGISTERS_ADDRESS: u16 = 0x108;
    pub const INPUT_REGISTERS_NB: u16 = 0x1;
    pub const INPUT_REGISTERS_TAB: &[u16] = &[0x000A];

    pub const REAL: f32 = 123456.00;

    pub const IREAL_ABCD: u32 = 0x0020F147;
    pub const IREAL_DCBA: u32 = 0x47F12000;
    pub const IREAL_BADC: u32 = 0x200047F1;
    pub const IREAL_CDAB: u32 = 0xF1470020;

    // pub const IREAL_ABCD: u32 = 0x47F12000);
    // pub const IREAL_DCBA: u32 = 0x0020F147;
    // pub const IREAL_BADC: u32 = 0xF1470020;
    // pub const IREAL_CDAB: u32 = 0x200047F1;
}

use unit_test_config::*;


#[derive(Eq, PartialEq)]
enum Backend {
    TCP,
    TCPPI,
    RTU,
}

fn run() -> Result<()> {
    let backend;
    let query;
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
    // Setup backend
    let mut modbus = match backend {
            Backend::TCP => {
                query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
                Modbus::new_tcp("127.0.0.1", 1502)
            },
            Backend::TCPPI => Modbus::new_tcp_pi("::0", "1502"),
            Backend::RTU => Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1),
        }.chain_err(|| "could not select backend")?;

    if backend == Backend::RTU {
        modbus.set_slave(SERVER_ID).chain_err(|| format!("could not set modbus slave address {}", SERVER_ID))?;
    }

    let header_lenght = modbus.get_header_length();

    let modbus_mapping =
        ModbusMapping::new_start_address(BITS_ADDRESS,
                                         BITS_NB,
                                         INPUT_BITS_ADDRESS,
                                         INPUT_BITS_NB,
                                         REGISTERS_ADDRESS,
                                         REGISTERS_NB,
                                         INPUT_REGISTERS_ADDRESS,
                                         INPUT_REGISTERS_NB).chain_err(|| "could not create modbus mapping")?;

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
            modbus.connect().chain_err(|| "could not connect")?;
        },
    }

    // loop {
    //     // let rc = modbus.receive(&mut query);
    //     // println!(">> {:?}", rc);
    // }

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
