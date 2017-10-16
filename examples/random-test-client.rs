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
const LOOPS: i32 = 1;    // Number of loops
#[allow(dead_code)] // only used in RTU context
const SERVER_ID: i32 = 17;   // Modbus ID Server
const ADDRESS_START: usize = 0;
const ADDRESS_END: usize = 99;

extern crate rand;
extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusClient};
use libmodbus_rs::errors::*;
use rand::Rng;


// At each loop the programm works in the range ADDRESS_START to ADDRESS_END
// then ADDRESS_START + 1 to ADDRESS_END and so on.
fn run() -> Result<()> {
    // Initialize random number generator
    let mut rng = rand::thread_rng();

    // // RTU
    // use libmodbus_rs::ModbusRTU;
    // let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 19200, 'N', 8, 1)?;
    // modbus.set_slave(SERVER_ID)?;

    // TCP
    use libmodbus_rs::ModbusTCP;
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;

    modbus.set_debug(true)?;

    modbus.connect()?;

    let mut num_bit = ADDRESS_END - ADDRESS_START;

    let mut request_bits = vec![0u8; num_bit];
    let response_bits = vec![0u8; num_bit];
    let mut request_registers = vec![0u16; num_bit];
    let mut response_registers = vec![0u16; num_bit];
    let mut rw_request_registers = vec![0u16; num_bit];

    let mut num_failures = 0;

    for _ in 0..LOOPS {
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
            let rc = modbus.write_bit(address as i32,
                                      match request_bits[0] {
                                          0 => false,
                                          _ => true,
                                      });
            match rc {
                Err(_) => {
                    // Error
                    println!("ERROR could not write_bit ({:?})", rc);
                    println!("Address = {}, value = {}", address, request_bits[0]);
                    num_failures += 1;
                },
                Ok(_len) => {
                    // let rc = modbus.read_bits(address as i32, 1, &mut response_bits);
                    // FIXME: refactor `read_` methods
                    let rc = modbus.read_bits(address as i32, 1);
                    match rc {
                        Err(_) => {
                            println!("ERROR could not read_bits single ({:?})", rc);
                            println!("address = {}", address);
                            num_failures += 1;
                        },
                        Ok(_) => {
                            if request_bits[0] != response_bits[0] {
                                println!("ERROR could not read_bits single ({:?})", rc);
                                println!("address = {}", address);
                                num_failures += 1;
                            }
                        },
                    }
                },
            }

            // MULTIPLE BITS
            let rc = modbus.write_bits(address as i32, num_bit as i32, &mut request_bits);
            match rc {
                Err(_) => {
                    println!("ERROR modbus_write_bits ({:?})", rc);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(len) => {
                    if len != num_bit as i32 {
                        println!("ERROR modbus_write_bits ({:?})", rc);
                        println!("Address = {}, num_bit = {}", address, num_bit);
                        num_failures += 1;
                    }
                    // FIXME: refactor `read_` methods
                    // let rc = modbus.read_bits(address as i32, num_bit as i32, &mut response_bits);
                    let rc = modbus.read_bits(address as i32, num_bit as i32);
                    match rc {
                        Err(_) => {
                            println!("ERROR modbus_read_bits ({:?})", rc);
                            println!("Address = {}, num_bit = {}", address, num_bit);
                            num_failures += 1;
                        },
                        Ok(len) => {
                            // FIXME: refactor `read_` methods
                            // if len != num_bit as i32 {
                            if len.len() == 0 {
                                println!("ERROR modbus_read_bits");
                                println!("Address = {}, num_bit = {}", address, num_bit);
                                num_failures += 1;
                            }
                            for i in 0..num_bit {
                                if response_bits[i] != request_bits[i] {
                                    println!("ERROR modbus_read_bits");
                                    println!("Address = {address}, value {request} (0x{request:X}) != {response} \
                                              (0x{response:X})",
                                             address = address,
                                             request = request_bits[i],
                                             response = response_bits[i]);
                                    num_failures += 1;
                                }
                            }
                        },
                    }
                },
            }

            //  SINGLE REGISTER
            let rc = modbus.write_register(address as i32, request_registers[0] as i32);
            match rc {
                Err(_) => {
                    println!("ERROR modbus_write_register ({:?})", rc);
                    println!("Address = {}, value = {request} (0x{request:X})",
                             address,
                             request = request_registers[0]);
                    num_failures += 1;
                },
                Ok(_len) => {
                    match modbus.read_registers(address as i32, 1) {
                        Err(err) => {
                            println!("ERROR modbus_read_registers single ({:?})", err);
                            println!("Address = {}", address);
                            num_failures += 1;
                        },
                        Ok(response_registers) => {
                            if request_registers[0] != response_registers[0] {
                                println!("ERROR modbus_read_registers single");
                                println!("Address = {}, value {request} (0x{request:X}) != {response} \
                                          (0x{response:X})",
                                         address,
                                         request = request_registers[0],
                                         response = response_registers[0]);
                                num_failures += 1;
                            }
                        },
                    }
                },
            }

            // MULTIPLE REGISTERS
            let rc = modbus.write_registers(address as i32, num_bit as i32, &mut request_registers);
            match rc {
                Err(_) => {
                    println!("ERROR modbus_write_registers ({:?})", rc);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(_len) => {
                    match modbus.read_registers(address as i32, num_bit as i32) {
                        Err(err) => {
                            println!("ERROR modbus_read_registers ({:?})", err);
                            println!("Address = {}, num_bit = {}", address, num_bit);
                            num_failures += 1;
                        },
                        Ok(response_registers) => {
                            for i in 0..num_bit {
                                if request_registers[i] != response_registers[i] {
                                    println!("ERROR modbus_read_registers");
                                    println!("Address = {}, value {request} (0x{request:X}) != {response} \
                                              (0x{response:X})",
                                             address,
                                             request = request_registers[i],
                                             response = response_registers[i]);
                                    num_failures += 1;
                                }
                            }
                        },
                    }
                },
            }

            // R/W MULTIPLE REGISTERS
            let rc = modbus.write_and_read_registers(address as i32,
                                                     num_bit as i32,
                                                     &mut rw_request_registers,
                                                     address as i32,
                                                     num_bit as i32,
                                                     &mut response_registers);
            match rc {
                Err(_) => {
                    println!("ERROR modbus_read_and_write_registers ({:?})", rc);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(_len) => {
                    for i in 0..num_bit {
                        if response_registers[i] != rw_request_registers[i] {
                            println!("ERROR modbus_read_and_write_registers READ");
                            println!("Address = {}, value {response} (0x{response:X}) != {request} (0x{request:X})",
                                     address,
                                     response = response_registers[i],
                                     request = rw_request_registers[i]);
                            num_failures += 1;
                        }
                    }

                    match modbus.read_registers(address as i32, num_bit as i32) {
                        Err(err) => {
                            println!("ERROR modbus_read_registers ({:?})", err);
                            println!("Address = {}, num_bit = {}", address, num_bit);
                            num_failures += 1;
                        },
                        Ok(response_registers) => {
                            for i in 0..num_bit {
                                if rw_request_registers[i] != response_registers[i] {
                                    println!("ERROR modbus_read_and_write_registers WRITE");
                                    println!("Address = {}, value {request} (0x{request:X}) != {response} \
                                              (0x{response:X})",
                                             address,
                                             request = rw_request_registers[i],
                                             response = response_registers[i]);
                                    num_failures += 1;
                                }
                            }
                        },
                    }
                },
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
