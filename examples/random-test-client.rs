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

use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};
use libmodbus_rs::errors::*;
use rand::Rng;


// At each loop the programm works in the range ADDRESS_START to ADDRESS_END
// then ADDRESS_START + 1 to ADDRESS_END and so on.
fn run() -> Result<()> {
    // Initialize random number generator
    let mut rng = rand::thread_rng();

    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;
    modbus.set_debug(true)?;

    // `accept()` and `listen()` are not working yeat?
    // let mut socket = modbus.tcp_listen(10)?;
    // modbus.tcp_accept(&mut socket)?;
    modbus.connect()?;

    let mut num_bit = ADDRESS_END - ADDRESS_START;

    let mut request_bits = vec![0u8; num_bit];
    let response_bits = vec![0u8; num_bit];
    let mut request_registers = vec![0u16; num_bit];
    let response_registers = vec![0u16; num_bit];
    let mut rw_request_registers = vec![0u16; num_bit];

    let mut num_failures = 0;

    for _ in 0..LOOPS {
        let mut address = ADDRESS_START;

        for address in ADDRESS_START..ADDRESS_END {
            // generate random numbers
            for i in 0..num_bit {
                // Random values for the request registers
                request_registers[i] = rng.gen::<u16>();
                // The rw_request_registers contain the bitwise not value of the corosponding request_registers
                rw_request_registers[i] = !request_registers[i];
                // request_bits contain the half value of the random request_registers value
                request_bits[i] = (request_registers[i] % 2) as u8;
            }

            num_bit = ADDRESS_END - address;

            // WRITE BIT
            match modbus.write_bit(address as i32, match request_bits[0] { 0 => true, _ => false } ) {
                Err(_) => {
                    println!("Error, could not write_bit()");
                    num_failures += 1;
                }
                Ok(_) => {
                    match modbus.read_bits(address as i32, 1, &mut request_bits) {
                        Err(_) => {
                            println!("Error, cound not read_bits() single");
                            num_failures += 1;
                        }
                        Ok(_) => {}
                    }
                }
            }

            // MULTIPLE BITS
            match modbus.write_bits(address as i32, num_bit as i32, &mut request_bits) {
                Ok(_) => {}
                Err(_) => {}
            }
        }
    }

    print!("Test: ");
    if num_failures > 0 {
        println!("{} FAILS", num_failures);
    } else {
        println!("SUCCESS");
    }

    Ok(())
}

fn main() {

    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
