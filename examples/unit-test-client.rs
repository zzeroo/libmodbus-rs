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
use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, ModbusTCPPI, ModbusRTU,
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
    print!("2/2 read_bits: ");
    assert_true!(rc.is_ok(), "FAILED (nb points {})", rc.unwrap());
    assert_true!(response_bits[0] == 1, "FAILED ({:0X} != {})",
                &response_bits[0], true);

    // End single

    // Multiple bits
    {
        let mut tab_value = vec![0u8; BITS_NB as usize];

        set_bits_from_bytes(&mut tab_value, 0, BITS_NB, BITS_TAB);
        let rc = modbus.write_bits(BITS_ADDRESS, BITS_NB, &tab_value).unwrap();
        print!("1/2 write_bits: ");
        assert_true!(rc == BITS_NB as i32, "");
    }

    let rc = modbus.read_bits(BITS_ADDRESS, BITS_NB, &mut response_bits).unwrap();
    print!("2/2 read_bits: ");
    assert_true!(rc == BITS_NB as i32, "FAILED (nb points {:?})", rc);

    let mut i: usize = 0;
    let mut nb_points = BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };

        let value = get_byte_from_bits(&response_bits, i as u8 * 8, nb_bits);
        assert_true!(value == BITS_TAB[i], "FAILED ({:0X} != {:0X})", value, BITS_TAB[i]);

        nb_points = nb_points - nb_bits;
        i = i + 1;
    }
    println!("OK");
    // End of multiple bits

    // DISCRETE INPUTS

    let rc = modbus.read_input_bits(INPUT_BITS_ADDRESS,
                                    INPUT_BITS_NB, &mut response_bits).unwrap();

    print!("1/1 modbus_read_input_bits: ");
    assert_true!(rc == INPUT_BITS_NB as i32, "FAILED (nb points {})", rc);

    let mut i = 0;
    let mut nb_points = INPUT_BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };
        let value = get_byte_from_bits(&response_bits, i as u8 * 8, nb_bits);
        assert_true!(value == INPUT_BITS_TAB[i], "FAILED ({:0X} != {:0X})",
                    value, INPUT_BITS_TAB[i]);

        nb_points = nb_points - nb_bits;
        i = i + 1;
    }
    println!("OK");

    // HOLDING REGISTERS

    // Single register
    let rc = modbus.write_register(REGISTERS_ADDRESS, 0x1234);
    print!("1/2 modbus_write_register: ");
    assert_true!(rc.is_ok(), "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS, 1, &mut response_registers).unwrap();
    print!("2/2 modbus_read_registers: ");
    assert_true!(rc == 1, "FAILED (nb points {:?})", rc);
    assert_true!(response_registers[0] == 0x1234, "FAILED ({:0x} != {:?})",
                response_registers[0], 0x1234);

    // End of single register

    // Many registers
    let rc = modbus.write_registers(REGISTERS_ADDRESS, REGISTERS_NB, REGISTERS_TAB).unwrap();
    print!("1/5 modbus_write_registers: ");
    assert_true!(rc == REGISTERS_NB as i32, "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS, REGISTERS_NB, &mut response_registers).unwrap();
    print!("2/5 modbus_read_registers: ");
    assert_true!(rc == REGISTERS_NB as i32, "FAILED (nb points {:?}", rc);







    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
