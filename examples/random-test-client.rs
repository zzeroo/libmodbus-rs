/// This is an example client, copied from the libmodbus test random-test-client.c
///
/// The goal of this program is to check all major functions of
///
/// libmodbus:
///
/// - write_coil
/// - read_bits
/// - write_coils
/// - write_register
/// - read_registers
/// - write_registers
/// - read_registers
///
/// All these functions are called with random values on a address range defined by the following consts
const LOOPS: i32            = 1;    // Number of loops
const SERVER_ID: i32        = 17;   // Modbus ID Server
const ADDRESS_START: usize  = 0;
const ADDRESS_END: usize    = 99;

extern crate rand;
extern crate libmodbus_rs;
#[macro_use] extern crate error_chain;

use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
use libmodbus_rs::errors::*;
use rand::Rng;


// At each loop the programm works in the range ADDRESS_START to ADDRESS_END
// then ADDRESS_START + 1 to ADDRESS_END and so on.
fn run() -> Result<()> {
    // Initialize random number generator
    let mut rng = rand::thread_rng();

    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let mut server_socket = modbus.tcp_listen(10).unwrap();
    modbus.set_debug(true)?;

    modbus.connect();
    // modbus.tcp_accept(&mut server_socket);

    let mut nb = ADDRESS_END - ADDRESS_START;

    let mut request_bits = vec![0u8; nb];
    let response_bits = vec![0u8; nb];
    let mut request_registers = vec![0u16; nb];
    let response_registers = vec![0u16; nb];
    let mut rw_request_registers = vec![0u16; nb];

    let num_fail = 0;

    for _ in 0..LOOPS {
        let mut address = ADDRESS_START;

        for address in ADDRESS_START..ADDRESS_END {
            for i in 0..nb {
                // Random values for the request registers
                request_registers[i] = rng.gen::<u16>();
                // The rw_request_registers contain the bitwise not value of the corosponding request_registers
                rw_request_registers[i] = !request_registers[i];
                // request_bits contain the half value of the random request_registers value
                request_bits[i] = (request_registers[i] % 2) as u8;
            }
            println!("request_registers for address: {}\n{:?}", &address, &request_registers);
            println!("rw_request_registers:\n{:?}", &rw_request_registers);
            println!("request_bits:\n{:?}", &request_bits);

            nb = ADDRESS_END - address;

            // Write bit
            let rc = modbus.write_bit(address as u8, match request_bits[0] { 0 => true, _ => false } );
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
