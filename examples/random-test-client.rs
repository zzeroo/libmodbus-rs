use libmodbus::{Modbus, ModbusClient};
use rand::Rng;


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

// At each loop the programm works in the range ADDRESS_START to ADDRESS_END
// then ADDRESS_START + 1 to ADDRESS_END and so on.
fn run() -> Result<(), String> {
    // Initialize random number generator
    let mut rng = rand::thread_rng();

    // // RTU
    // use libmodbus::ModbusRTU;
    // let mut modbus = Modbus::new_rtu("/dev/ttyUSB0", 19200, 'N', 8, 1).expect("could not create RTU context");
    // modbus.set_slave(SERVER_ID).expect("could not set slave ID");

    // TCP
    use libmodbus::ModbusTCP;
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("could not create TCP context");

    modbus.set_debug(true).expect("could not set DEBUG mode");
    modbus.connect().expect("could not coonect");

    let mut num_bit = ADDRESS_END - ADDRESS_START;

    let mut request_bits = vec![0u8; num_bit];
    let mut response_bits = vec![0u8; num_bit];
    let mut request_registers = vec![0u16; num_bit];
    let mut response_registers = vec![0u16; num_bit];
    let mut rw_request_registers = vec![0u16; num_bit];

    let mut num_failures = 0;

    for _ in 0..LOOPS {
        for address in ADDRESS_START..ADDRESS_END {

            for i in 0..num_bit {
                // Random numbers (short)
                // fill `request_registers` with random u16 values
                request_registers[i] = rng.gen::<u16>();
                // `rw_request_registers` conaint the invert `request_registers` values
                rw_request_registers[i] = !request_registers[i];
                // Modulo2 only 0, 1/ true, false values
                request_bits[i] = (request_registers[i] % 2) as u8;
            }

            num_bit = ADDRESS_END - address;

            // WRITE BIT
            match modbus.write_bit(address as u16,
                                   match request_bits[0] {
                                       0 => false,
                                       _ => true,
                                   }) {
                Err(err) => {
                    // Error
                    println!("ERROR write_bit: '{}'", err);
                    println!("Address = {}, value = {}", address, request_bits[0]);
                    num_failures += 1;
                },
                Ok(_) => {
                    match modbus.read_bits(address as u16, 1, &mut response_bits) {
                        Err(err) => {
                            println!("ERROR read_bits single: '{}')", err);
                            println!("address = {}", address);
                            num_failures += 1;
                        },
                        Ok(len) => {
                            if request_bits[0] != response_bits[0] {
                                println!("ERROR read_bits single ({})", len);
                                println!("address = {}", address);
                                num_failures += 1;
                            }
                        },
                    }
                },
            }

            // MULTIPLE BITS
            match modbus.write_bits(address as u16, num_bit as u16, &request_bits) {
                Err(err) => {
                    println!("ERROR write_bits: '{}'", err);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(_) => {
                    match modbus.read_bits(address as u16, num_bit as u16, &mut response_bits) {
                        Err(err) => {
                            println!("ERROR read_bits: '{}'", err);
                            num_failures += 1;
                        },
                        Ok(len) => {
                            if len != num_bit as u16 {
                                println!("ERROR read_bits");
                                println!("Address = {}, num_bit = {}", address, num_bit);
                                num_failures += 1;
                            } else {
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
                            }
                        },
                    }
                },
            }

            //  SINGLE REGISTER
            match modbus.write_register(address as u16, request_registers[0]) {
                Err(err) => {
                    println!("ERROR write_register: '{}'", err);
                    println!("Address = {}, value = {request} (0x{request:X})",
                             address,
                             request = request_registers[0]);
                    num_failures += 1;
                },
                Ok(_) => {
                    match modbus.read_registers(address as u16, 1, &mut response_registers) {
                        Err(err) => {
                            println!("ERROR read_registers single: '{}'", err);
                            println!("Address = {}", address);
                            num_failures += 1;
                        },
                        Ok(_len) => {
                            if request_registers[0] != response_registers[0] {
                                println!("ERROR read_registers single");
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
            match modbus.write_registers(address as u16, num_bit as u16, &request_registers) {
                Err(err) => {
                    println!("ERROR write_registers:('{}'", err);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(len) => {
                    if len != num_bit as u16 {
                        println!("ERROR write_registers: ({})", len);
                        println!("Address = {}, num_bit = {}", address, num_bit);
                        num_failures += 1;
                    } else {
                        match modbus.read_registers(address as u16, num_bit as u16, &mut response_registers) {
                            Err(err) => {
                                println!("ERROR read_registers: '{}'", err);
                                println!("Address = {}, num_bit = {}", address, num_bit);
                                num_failures += 1;
                            },
                            Ok(len) => {
                                if len != num_bit as u16 {
                                    println!("ERROR read_registers: ({})", len);
                                    println!("Address = {}, num_bit = {}", address, num_bit);
                                    num_failures += 1;
                                } else {
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
                                }
                            },
                        }
                    }
                },
            }

            // R/W MULTIPLE REGISTERS
            match modbus.write_and_read_registers(address as u16,
                                                  num_bit as u16,
                                                  &mut rw_request_registers,
                                                  address as u16,
                                                  num_bit as u16,
                                                  &mut response_registers) {
                Err(err) => {
                    println!("ERROR read_and_write_registers: '{}'", err);
                    println!("Address = {}, num_bit = {}", address, num_bit);
                    num_failures += 1;
                },
                Ok(len) => {
                    if len != num_bit as u16 {
                        println!("ERROR read_and_write_registers: '{}'", len);
                        println!("Address = {}, num_bit = {}", address, num_bit);
                        num_failures += 1;
                    } else {
                        for i in 0..num_bit {
                            if response_registers[i] != rw_request_registers[i] {
                                println!("ERROR read_and_write_registers READ");
                                println!("Address = {}, value {response} (0x{response:X}) != {request} \
                                          (0x{request:X})",
                                         address,
                                         response = response_registers[i],
                                         request = rw_request_registers[i]);
                                num_failures += 1;
                            }
                        }
                    }

                    match modbus.read_registers(address as u16, num_bit as u16, &mut response_registers) {
                        Err(err) => {
                            println!("ERROR modbus_read_registers ({:?})", err);
                            println!("Address = {}, num_bit = {}", address, num_bit);
                            num_failures += 1;
                        },
                        Ok(_len) => {
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
    } // End LOOP

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
