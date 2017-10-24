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
use libmodbus_rs::{Modbus, ModbusMapping, ModbusServer, ModbusClient, ModbusTCP, ModbusTCPPI, ModbusRTU,
                   ErrorRecoveryMode};
use libmodbus_rs::prelude::*;
use std::env;

const EXCEPTION_RC: u32 = 2;

// https://play.rust-lang.org/?gist=0eb013f83a727b79b04598101ec196d5&version=stable
macro_rules! bug_report {
    ( $cond:expr, $format:expr $(, $args:expr)*) => {
        // panic! for real code
        let format = format!($format, $($args),*);
        println!("\nLine: {}: assertion error for '{}': {}\n", line!(), stringify!($cond), format);
    }
}
macro_rules! assert_true {
    ( $cond:expr, $format:expr $(, $args:expr)*) => {
        if $cond {
            println!("OK");
        } else {
            bug_report!($cond, $format $(, $args)* );
        }
    }
}

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
                println!("Usage:\n  {} [tcp|tcppi|rtu] - Modbus server for unit testing\n\n",
                         args[0]);
                std::process::exit(-1);
            }
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
            }
            Backend::TCPPI => {
                query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
                Modbus::new_tcp_pi("::0", "1502")
            }
            Backend::RTU => {
                query = vec![0u8; Modbus::RTU_MAX_ADU_LENGTH as usize];
                Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1)
            }
        }.chain_err(|| "could not select backend")?;

    modbus.set_debug(true).expect("could not set modbus DEBUG mode");
    modbus.set_error_recovery(Some(&[ErrorRecoveryMode::Link, ErrorRecoveryMode::Protocol]))
        .expect("could not set error recovery mode");

    if backend == Backend::RTU {
        modbus.set_slave(SERVER_ID)
            .chain_err(|| format!("could not set modbus slave address {}", SERVER_ID))?;
    }

    let old_response_timeout = modbus.get_response_timeout()
        .expect("could not get response timeout");
    match modbus.connect() {
        Err(err) => panic!("Connection failed: {}", err),
        Ok(_) => {}
    }
    let new_response_timeout = modbus.get_response_timeout()
        .expect("could not get response timeout");

    println!("** UNIT TESTING **");

    print!("1/1 No response timeout modification on connect: ");
    assert_true!(old_response_timeout == new_response_timeout, "");

    // Allocate and initialize the memory to store the bits
    let nb_points = if BITS_NB > INPUT_BITS_NB { BITS_NB } else { INPUT_BITS_NB };
    let mut response_bits = vec![0u8; nb_points as usize];

    // Allocate and initialize the memory to store the registers
    let nb_points = if REGISTERS_NB > INPUT_REGISTERS_NB { REGISTERS_NB } else { INPUT_REGISTERS_NB };
    let mut response_registers = vec![0u16; nb_points as usize];

    println!("\nTEST WRITE/READ:");

    // COIL BITS

    // Single

    let rc = modbus.write_bit(BITS_ADDRESS, true);
    print!("1/2 write_bit: ");
    assert_true!(rc.is_ok(), "");

    let rc = modbus.read_bits(BITS_ADDRESS, 1, &mut response_bits);
    print!("2/2 modbus_read_bits: ");
    assert_true!(rc.is_ok(), "FAILED (nb points {})", rc.unwrap());
    assert_true!(response_bits[0] == 1, "FAILED ({:0X} != {})", &response_bits[0], true);

    // End single

    // Multiple bits
    let mut value = vec![0u8; BITS_NB as usize];

    set_bits_from_bytes(&mut value, BITS_ADDRESS, BITS_NB, &response_bits);
    print!("1/2 modbus_write_bits: ");


    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
