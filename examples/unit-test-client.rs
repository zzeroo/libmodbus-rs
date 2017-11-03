extern crate libmodbus_rs;

mod unit_test_config;

use libmodbus_rs::errors::*;
use libmodbus_rs::prelude::*;
use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, ModbusTCPPI, ModbusRTU,
                   ErrorRecoveryMode};
use std::env;
use unit_test_config::*;

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

fn equal_dword(tab_reg: &[u16], value: u32) -> bool {
    tab_reg[0] as u32 == (value >> 16) && tab_reg[1] as u32 == (value & 0xFFFF)
}

fn run() -> Result<()> {
    let backend;

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
                Modbus::new_tcp("127.0.0.1", 1502)
            }
            Backend::TCPPI => {
                Modbus::new_tcp_pi("::0", "1502")
            }
            Backend::RTU => {
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
    assert_true!(rc.is_ok(), "FAILED (nb_points {})", rc.unwrap());
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
    assert_true!(rc == BITS_NB as i32, "FAILED (nb_points {:?})", rc);

    let mut i: usize = 0;
    let mut nb_points = BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };

        let value = get_byte_from_bits(&response_bits, i as u8 * 8, nb_bits);
        assert_true!(value == BITS_TAB[i], "FAILED ({:0X} != {:0X})", value, BITS_TAB[i]);

        nb_points -= nb_bits;
        i += 1;
    }
    println!("OK");
    // End of multiple bits

    // DISCRETE INPUTS

    let rc = modbus.read_input_bits(INPUT_BITS_ADDRESS,
                                    INPUT_BITS_NB, &mut response_bits).unwrap();

    print!("1/1 modbus_read_input_bits: ");
    assert_true!(rc == INPUT_BITS_NB as i32, "FAILED (nb_points {})", rc);

    let mut i = 0;
    let mut nb_points = INPUT_BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };
        let value = get_byte_from_bits(&response_bits, i as u8 * 8, nb_bits);
        assert_true!(value == INPUT_BITS_TAB[i], "FAILED ({:0X} != {:0X})",
                    value, INPUT_BITS_TAB[i]);

        nb_points -= nb_bits;
        i += 1;
    }
    println!("OK");

    // HOLDING REGISTERS

    // Single register
    let rc = modbus.write_register(REGISTERS_ADDRESS, 0x1234);
    print!("1/2 modbus_write_register: ");
    assert_true!(rc.is_ok(), "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS, 1, &mut response_registers).unwrap();
    print!("2/2 modbus_read_registers: ");
    assert_true!(rc == 1, "FAILED (nb_points {:?})", rc);
    assert_true!(response_registers[0] == 0x1234, "FAILED ({:0x} != {:?})",
                response_registers[0], 0x1234);

    // End of single register

    // Many registers
    let rc = modbus.write_registers(REGISTERS_ADDRESS, REGISTERS_NB, REGISTERS_TAB).unwrap();
    print!("1/5 modbus_write_registers: ");
    assert_true!(rc == REGISTERS_NB as i32, "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS, REGISTERS_NB, &mut response_registers).unwrap();
    print!("2/5 modbus_read_registers: ");
    assert_true!(rc == REGISTERS_NB as i32, "FAILED (nb_points {:?}", rc);

    for i in 0..REGISTERS_NB as usize {
        assert_true!(response_registers[i] == REGISTERS_TAB[i],
                    "FAILED ({:?} != {:?})",
                    response_registers[i], REGISTERS_TAB[i]);
    }

    let rc = modbus.read_registers(REGISTERS_ADDRESS, 0, &mut response_registers);
    print!("3/5 modbus_read_registers (0): ");
    assert_true!(rc.is_err(), "FAILED (nb_points {})", 0);

    let nb_points = if REGISTERS_NB > INPUT_REGISTERS_NB { REGISTERS_NB } else { INPUT_REGISTERS_NB };
    let mut response_registers = vec![0u16; nb_points as usize];

    // write registers to zero from `response_registers` and store read registers
    // into `response_registers`. So the read regiserts mut set to 0, except the
    // first one because there is an offset of 1 register on write.
    let rc = modbus.write_and_read_registers(REGISTERS_ADDRESS + 1,
                                            REGISTERS_NB - 1,
                                            &response_registers.clone(),
                                            REGISTERS_ADDRESS,
                                            REGISTERS_NB,
                                            &mut response_registers).unwrap();

    print!("4/5 modbus_write_and_read_registers: ");
    assert_true!(rc == REGISTERS_NB as i32, "FAILED (nb_points {} != {})", rc, REGISTERS_NB);

    assert_true!(response_registers[0] == REGISTERS_TAB[0],
            "FAILED ({:?} != {:?})", response_registers[0], REGISTERS_TAB[0]);

    for i in 1..REGISTERS_NB as usize {
        assert_true!(response_registers[i] == 0,
            "FAILED ({:0X} != {:0X})", response_registers[i], 0);
    }

    // End of many registers


    // INPUT REGISTERS
    let rc = modbus.read_input_registers(INPUT_REGISTERS_ADDRESS,
        INPUT_REGISTERS_NB,
        &mut response_registers).unwrap();

    print!("1/1 modbus_read_input_registers: ");
    assert_true!(rc == INPUT_REGISTERS_NB as i32, "FAILED (nb_points {})", rc);

    for i in 0..INPUT_REGISTERS_NB as usize {
         assert_true!(response_registers[i] == INPUT_REGISTERS_TAB[i],
                     "FAILED ({:0X} != {:0X})",
                     response_registers[i], INPUT_REGISTERS_TAB[i]);
    }

    // MASKS
    print!("1/1 Write mask: ");
    let _rc = modbus.write_register(REGISTERS_ADDRESS, 0x12).unwrap();
    let rc = modbus.mask_write_register(REGISTERS_ADDRESS, 0xF2, 0x25);
    assert_true!(rc.is_ok(), "FAILED ({:?} == -1)", rc);
    let rc = modbus.read_registers(REGISTERS_ADDRESS, 1, &mut response_registers).unwrap();
    assert_true!(response_registers[0] == 0x17,
                "FAILED ({:0X} != {:0X})",
                response_registers[0], 0x17);

    print!("\nTEST FLOATS");
    // FLOAT
    print!("1/4 Set/get float ABCD: ");
    set_float_abcd(REAL, &mut response_registers);
    assert_true!(equal_dword(&response_registers, IREAL_ABCD), "FAILED Set float ABCD");
    let real = get_float_abcd(&response_registers[0..2]);
    assert_true!(real == REAL, "FAILED ({} != {})", real, REAL);

    print!("2/4 Set/get float DCBA: ");
    set_float_dcba(REAL, &mut response_registers);
    assert_true!(equal_dword(&response_registers, IREAL_DCBA), "FAILED Set float DCBA");
    let real = get_float_dcba(&response_registers[0..2]);
    assert_true!(real == REAL, "FAILED ({} != {})", real, REAL);

    print!("3/4 Set/get float BADC: ");
    set_float_badc(REAL, &mut response_registers);
    assert_true!(equal_dword(&response_registers, IREAL_BADC), "FAILED Set float BADC");
    let real = get_float_badc(&response_registers[0..2]);
    assert_true!(real == REAL, "FAILED ({} != {})", real, REAL);

    print!("4/4 Set/get float CDAB: ");
    set_float_cdab(REAL, &mut response_registers);
    assert_true!(equal_dword(&response_registers, IREAL_CDAB), "FAILED Set float CDAB");
    let real = get_float_cdab(&response_registers[0..2]);
    assert_true!(real == REAL, "FAILED ({} != {})", real, REAL);

    print!("\nAt this point, error messages doesn't mean the test has failed");

    // ILLEGAL DATA ADDRESS
    print!("\nTEST ILLEGAL DATA ADDRESS:");

    /* The mapping begins at the defined addresses and ends at address +
    * nb_points so these addresses are not valid. */

    let rc = modbus.read_bits(0, 1, &mut response_bits);
    print!("* read_bits (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_bits(BITS_ADDRESS, BITS_NB + 1, &mut response_bits);
    print!("* read_bits (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_bits(0, 1, &mut response_bits);
    print!("* read_input_bits (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_bits(INPUT_BITS_ADDRESS,
                                INPUT_BITS_NB + 1, &mut response_bits);
    print!("* read_input_bits (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_registers(0, 1, &mut response_registers);
    print!("* read_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS,
                               REGISTERS_NB_MAX + 1, &mut response_registers);
    print!("* read_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_registers(0, 1, &mut response_registers);
    print!("* read_input_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_registers(INPUT_REGISTERS_ADDRESS,
                                     INPUT_REGISTERS_NB + 1,
                                     &mut response_registers);
    print!("* read_input_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bit(0, true);
    print!("* write_bit (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bit(BITS_ADDRESS + BITS_NB, true);
    print!("* write_bit (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bits(0, 1, &mut response_bits);
    print!("* write_coils (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bits(BITS_ADDRESS + BITS_NB,
                           BITS_NB, &mut response_bits);
    print!("* write_coils (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_register(0, response_registers[0]);
    print!("* write_register (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_register(REGISTERS_ADDRESS + REGISTERS_NB_MAX,
                                response_registers[0]);
    print!("* write_register (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_registers(0, 1, &mut response_registers);
    print!("* write_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_registers(REGISTERS_ADDRESS + REGISTERS_NB_MAX,
                                REGISTERS_NB, &mut response_registers);
    print!("* write_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.mask_write_register(0, 0xF2, 0x25);
    print!("* mask_write_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.mask_write_register(REGISTERS_ADDRESS + REGISTERS_NB_MAX,
                                    0xF2, 0x25);
    print!("* mask_write_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_and_read_registers(0, 1, &response_registers.clone(), 0, 1, &mut response_registers);
    print!("* write_and_read_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_and_read_registers(REGISTERS_ADDRESS + REGISTERS_NB_MAX,
                                         REGISTERS_NB, &response_registers.clone(),
                                         REGISTERS_ADDRESS + REGISTERS_NB_MAX,
                                         REGISTERS_NB, &mut response_registers);
    print!("* write_and_read_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    // TOO MANY DATA
    print!("\nTOO MANY DATA ERROR:\n");

    let rc = modbus.read_bits(BITS_ADDRESS,
                          Modbus::MAX_READ_BITS as u16 + 1, &mut response_bits);
    print!("* read_bits: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_input_bits(INPUT_BITS_ADDRESS,
                                Modbus::MAX_READ_BITS as u16 + 1, &mut response_bits);
    print!("* read_input_bits: ");
    // assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_registers(REGISTERS_ADDRESS,
                               Modbus::MAX_READ_REGISTERS as u16 + 1,
                               &mut response_registers);
    print!("* read_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_input_registers(INPUT_REGISTERS_ADDRESS,
                                     Modbus::MAX_READ_REGISTERS as u16 + 1,
                                     &mut response_registers);
    print!("* read_input_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.write_bits(BITS_ADDRESS,
                           Modbus::MAX_WRITE_BITS as u16 + 1, &mut response_bits);
    print!("* write_bits: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.write_registers(REGISTERS_ADDRESS,
                                Modbus::MAX_WRITE_REGISTERS as u16 + 1,
                                &mut response_registers);
    print!("* write_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    // SLAVE REPLY
    let old_slave = modbus.get_slave();

    modbus.set_slave(INVALID_SERVER_ID);
    let rc = modbus.read_registers(REGISTERS_ADDRESS,
                                REGISTERS_NB, &mut response_registers);

    if backend == Backend::RTU {
        const RAW_REQ_LENGTH: i32 = 6;
        let mut raw_req = vec![INVALID_SERVER_ID, 0x03, 0x00, 0x01, 0x01, 0x01];
        /* Too many points */
        let req_invalid_req = vec![INVALID_SERVER_ID, 0x03, 0x00, 0x01, 0xFF, 0xFF];
        const RAW_RSP_LENGTH: i32 = 7;
        let mut raw_rsp = vec![INVALID_SERVER_ID, 0x03, 0x04, 0, 0, 0, 0];
        let mut rsp = vec![0; Modbus::RTU_MAX_ADU_LENGTH];

        /* No response in RTU mode */
        print!("1-A/3 No response from slave {}: ", INVALID_SERVER_ID);
        assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Timeout", "");

        /* The slave raises a timeout on a confirmation to ignore because if an
         * indication for another slave is received, a confirmation must follow */


        /* Send a pair of indication/confirmation to the slave with a different
         * slave ID to simulate a communication on a RS485 bus. At first, the
         * slave will see the indication message then the confirmation, and it must
         * ignore both. */
        modbus.send_raw_request(&mut raw_req, RAW_REQ_LENGTH * std::mem::size_of::<u8>() as i32);
        modbus.send_raw_request(&mut raw_rsp, RAW_RSP_LENGTH * std::mem::size_of::<u8>() as i32);
        let rc = modbus.receive_confirmation(&mut rsp);

        print!("1-B/3 No response from slave {} on indication/confirmation messages: ",
               INVALID_SERVER_ID);
        assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Timeout", "");

        // /* Send an INVALID request for another slave */
        // modbus.send_raw_request(raw_invalid_req, RAW_REQ_LENGTH * sizeof(uint8_t));
        // let rc = modbus.receive_confirmation(rsp);
        //
        // print!("1-C/3 No response from slave {} with invalid request: ",
        //        INVALID_SERVER_ID);
        // assert_true!(rc.is_err() && errno == ETIMEDOUT, "");
        //
        // let rc = modbus.set_slave(MODBUS_BROADCAST_ADDRESS);
        // assert_true!(rc != -1, "Invalid broadcast address");
        //
        // let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
        //                            UT_REGISTERS_NB, tab_rp_registers);
        // print!("2/3 No reply after a broadcast query: ");
        // assert_true!(rc.is_err() && errno == ETIMEDOUT, "");
    } else {
    }










































    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
