extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer, ModbusMapping};
use libmodbus_rs::errors::*; // for the `Result<T>` type
use std::env;


mod unit_test_config {
    pub const SERVER_ID: i32         = 17;
    pub const INVALID_SERVER_ID: i32 = 18;

    pub const UT_BITS_ADDRESS: u16 = 0x130;
    pub const UT_BITS_NB: u16 = 0x25;
    //let UT_BITS_TAB: Vec<u8> = vec!{ 0xCD, 0x6B, 0xB2, 0x0E, 0x1B };

    pub const UT_INPUT_BITS_ADDRESS: u16 = 0x1C4;
    pub const UT_INPUT_BITS_NB: u16 = 0x16;
    //let UT_INPUT_BITS_TAB: Vec<u8> = vec!{ 0xAC, 0xDB, 0x35 };

    pub const UT_REGISTERS_ADDRESS: u16 = 0x160;
    pub const UT_REGISTERS_NB: u16 = 0x3;
    pub const UT_REGISTERS_NB_MAX: u16 = 0x20;
    //let UT_REGISTERS_TAB: Vec<u16> = vec!{ 0x022B, 0x0001, 0x0064 };

    /* Raise a manual exception when this address is used for the first byte */
    pub const UT_REGISTERS_ADDRESS_SPECIAL: u16 = 0x170;
    /* The response of the server will contains an invalid TID or slave */
    pub const UT_REGISTERS_ADDRESS_INVALID_TID_OR_SLAVE: u16 = 0x171;
    /* The server will wait for 1 second before replying to test timeout */
    pub const UT_REGISTERS_ADDRESS_SLEEP_500_MS: u16 = 0x172;
    /* The server will wait for 5 ms before sending each byte */
    pub const UT_REGISTERS_ADDRESS_BYTE_SLEEP_5_MS: u16 = 0x173;

    /* If the following value is used, a bad response is sent.
       It's better to test with a lower value than
       UT_REGISTERS_NB_POINTS to try to raise a segfault. */
    pub const UT_REGISTERS_NB_SPECIAL: u16 = 0x2;

    pub const UT_INPUT_REGISTERS_ADDRESS: u16 = 0x108;
    pub const UT_INPUT_REGISTERS_NB: u16 = 0x1;
    //let UT_INPUT_REGISTERS_TAB: Vec<u16> = vec!{ 0x000A };

    pub const UT_REAL: f32 = 123456.00;

    pub const UT_IREAL_ABCD: u32 = 0x0020F147;
    pub const UT_IREAL_DCBA: u32 = 0x47F12000;
    pub const UT_IREAL_BADC: u32 = 0x200047F1;
    pub const UT_IREAL_CDAB: u32 = 0xF1470020;

    /* pub const UT_IREAL_ABCD: u32 = 0x47F12000);
    pub const UT_IREAL_DCBA: u32 = 0x0020F147;
    pub const UT_IREAL_BADC: u32 = 0xF1470020;
    pub const UT_IREAL_CDAB: u32 = 0x200047F1;*/
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
            }
        }
    } else {
        // By default
        backend = Backend::TCP;
    }
    // Setup backend
    let mut modbus = match backend {
        Backend::TCP => {
            use libmodbus_rs::ModbusTCP;

            Modbus::new_tcp("127.0.0.1", 1502)
        }
        Backend::TCPPI => {
            use libmodbus_rs::ModbusTCPPI;

            Modbus::new_tcp_pi("::0", "1502")
        }
        Backend::RTU => {
            use libmodbus_rs::ModbusRTU;

            Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1)
        }
    }?;

    if backend == Backend::RTU { modbus.set_slave(SERVER_ID); }

    let header_lenght = modbus.get_header_length();
    println!("{:?}", header_lenght);

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500)?;

    println!(">> {:?}", modbus_mapping);

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
