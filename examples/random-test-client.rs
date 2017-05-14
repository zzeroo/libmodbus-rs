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
    // modbus.set_debug(true)?;

    // `accept()` and `listen()` are not working yeat?
    // let mut socket = modbus.tcp_listen(10)?;
    // modbus.tcp_accept(&mut socket)?;
    modbus.connect()?;

    let mut num_bit = ADDRESS_END - ADDRESS_START;

    let mut request_bits = vec![0u8; num_bit];
    let mut response_bits = vec![0u8; num_bit];
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
            let rc = modbus.write_bit(address as i32, match request_bits[0] { 0 => false, _ => true } );
            match rc {
                Err(_) => { // Error
                    println!("ERROR could not write_bit ({:?})", &rc);
                    println!("Address = {}, value = {}", address, request_bits[0]);
                    num_failures += 1;
                }
                Ok(_) => {
                    let rc = modbus.read_bits(address as i32, 1, &mut response_bits);
                    match rc {
                        Err(_) => {
                            println!("ERROR could not read_bits single ({:?})", &rc);
                            println!("address = {}", address);
                            num_failures += 1;
                        }
                        Ok(_) => {
                            if request_bits[0] != response_bits[0] {
                                println!("ERROR could not read_bits single ({:?})", &rc);
                                println!("address = {}", address);
                                num_failures += 1;
                            }
                        }
                    }
                }
                _ => {}
            }

            // // MULTIPLE BITS
            // let rc = modbus.write_bits(address as i32, num_bit as i32, &mut request_bits)?;
            // match rc {
            //     -1 => {
            //         println!("ERROR modbus_write_bits ({:?})", rc);
            //         println!("Address = {}, num_bit = {}", address, num_bit);
            //         num_failures += 1;
            //     }
            //     _ => {
            //         let rc = modbus.read_bits(address as i32, num_bit as i32, &mut request_bits)?;
            //         match rc {
            //             -1 => {
            //                 println!("ERROR modbus_read_bits");
            //                 println!("Address = {}, num_bit = {}", address, num_bit);
            //                 num_failures += 1;
            //             }
            //             _ => {
            //                 for i in 0..num_bit {
            //                     if response_bits[i] != request_bits[i] {
            //                         println!("ERROR modbus_read_bits");
            //                         println!("Address = {address}, value {request} (0x{request:X}) != {response} (0x{response:X})",
            //                             address=address, request=request_bits[i], response=response_bits[i]);
            //                         num_failures += 1;
            //                     }
            //                 }
            //             }
            //         }
            //     }
            // }
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
