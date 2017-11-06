extern crate libmodbus_rs;

mod unit_test_config;

use libmodbus_rs::errors::*;
use libmodbus_rs::prelude::*;
use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, ModbusTCPPI, ModbusRTU,
                   Exception, FunctionCode, Timeout, ErrorRecoveryMode};
use std::env;
use unit_test_config::*;
use std::thread::sleep;
use std::time::Duration;

const EXCEPTION_RC: u16 = 2;

// https://play.rust-lang.org/?gist=0eb013f83a727b79b04598101ec196d5&version=stable
macro_rules! bug_report {
    ( $cond:expr, $format:expr $(, $args:expr)*) => {
        // panic! for real code
        let format = format!($format, $($args),*);
        print!("\nLine: {}: assertion error for '{}': {}\n", line!(), stringify!($cond), format);
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
    const NB_REPORT_SLAVE_ID: usize = 10;
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
    let nb_points = if UT_BITS_NB > UT_INPUT_BITS_NB { UT_BITS_NB } else { UT_INPUT_BITS_NB };
    let mut tab_rp_bits = vec![0u8; nb_points as usize];

    // Allocate and initialize the memory to store the registers
    let nb_points = if UT_REGISTERS_NB > UT_INPUT_REGISTERS_NB { UT_REGISTERS_NB } else { UT_INPUT_REGISTERS_NB };
    let mut tab_rp_registers = vec![0u16; nb_points as usize];

    println!("\nTEST WRITE/READ:");

    // COIL BITS

    // Single

    let rc = modbus.write_bit(UT_BITS_ADDRESS, true);
    print!("1/2 write_bit: ");
    assert_true!(rc.is_ok(), "");

    let rc = modbus.read_bits(UT_BITS_ADDRESS, 1, &mut tab_rp_bits);
    print!("2/2 read_bits: ");
    assert_true!(rc.is_ok(), "FAILED (nb_points {})", rc.unwrap());
    assert_true!(tab_rp_bits[0] == 1, "FAILED ({:0X} != {})",
                &tab_rp_bits[0], true);

    // End single

    // Multiple bits
    {
        let mut tab_value = vec![0u8; UT_BITS_NB as usize];

        set_bits_from_bytes(&mut tab_value, 0, UT_BITS_NB, UT_BITS_TAB);
        let rc = modbus.write_bits(UT_BITS_ADDRESS, UT_BITS_NB, &tab_value).unwrap();
        print!("1/2 write_bits: ");
        assert_true!(rc == UT_BITS_NB, "");
    }

    let rc = modbus.read_bits(UT_BITS_ADDRESS, UT_BITS_NB, &mut tab_rp_bits).unwrap();
    print!("2/2 read_bits: ");
    assert_true!(rc == UT_BITS_NB, "FAILED (nb_points {:?})", rc);

    let mut i: usize = 0;
    let mut nb_points = UT_BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };

        let value = get_byte_from_bits(&tab_rp_bits, i as u8 * 8, nb_bits);
        assert_true!(value == UT_BITS_TAB[i], "FAILED ({:0X} != {:0X})", value, UT_BITS_TAB[i]);

        nb_points -= nb_bits;
        i += 1;
    }
    println!("OK");
    // End of multiple bits

    // DISCRETE INPUTS

    let rc = modbus.read_input_bits(UT_INPUT_BITS_ADDRESS,
                                    UT_INPUT_BITS_NB, &mut tab_rp_bits).unwrap();

    print!("1/1 modbus_read_input_bits: ");
    assert_true!(rc == UT_INPUT_BITS_NB, "FAILED (nb_points {})", rc);

    let mut i = 0;
    let mut nb_points = UT_INPUT_BITS_NB;
    while nb_points > 0 {
        let nb_bits = if nb_points > 8 { 8 } else { nb_points };
        let value = get_byte_from_bits(&tab_rp_bits, i as u8 * 8, nb_bits);
        assert_true!(value == UT_INPUT_BITS_TAB[i], "FAILED ({:0X} != {:0X})",
                    value, UT_INPUT_BITS_TAB[i]);

        nb_points -= nb_bits;
        i += 1;
    }
    println!("OK");

    // HOLDING REGISTERS

    // Single register
    let rc = modbus.write_register(UT_REGISTERS_ADDRESS, 0x1234);
    print!("1/2 modbus_write_register: ");
    assert_true!(rc.is_ok(), "");

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS, 1, &mut tab_rp_registers).unwrap();
    print!("2/2 modbus_read_registers: ");
    assert_true!(rc == 1, "FAILED (nb_points {:?})", rc);
    assert_true!(tab_rp_registers[0] == 0x1234, "FAILED ({:0x} != {:?})",
                tab_rp_registers[0], 0x1234);

    // End of single register

    // Many registers
    let rc = modbus.write_registers(UT_REGISTERS_ADDRESS, UT_REGISTERS_NB, UT_REGISTERS_TAB).unwrap();
    print!("1/5 modbus_write_registers: ");
    assert_true!(rc == UT_REGISTERS_NB, "");

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS, UT_REGISTERS_NB, &mut tab_rp_registers).unwrap();
    print!("2/5 modbus_read_registers: ");
    assert_true!(rc == UT_REGISTERS_NB, "FAILED (nb_points {:?}", rc);

    for i in 0..UT_REGISTERS_NB as usize {
        assert_true!(tab_rp_registers[i] == UT_REGISTERS_TAB[i],
                    "FAILED ({:?} != {:?})",
                    tab_rp_registers[i], UT_REGISTERS_TAB[i]);
    }

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS, 0, &mut tab_rp_registers);
    print!("3/5 modbus_read_registers (0): ");
    assert_true!(rc.is_err(), "FAILED (nb_points {})", 0);

    let nb_points = if UT_REGISTERS_NB > UT_INPUT_REGISTERS_NB { UT_REGISTERS_NB } else { UT_INPUT_REGISTERS_NB };
    let mut tab_rp_registers = vec![0u16; nb_points as usize];

    // write registers to zero from `tab_rp_registers` and store read registers
    // into `tab_rp_registers`. So the read regiserts mut set to 0, except the
    // first one because there is an offset of 1 register on write.
    let rc = modbus.write_and_read_registers(UT_REGISTERS_ADDRESS + 1,
                                            UT_REGISTERS_NB - 1,
                                            &tab_rp_registers.clone(),
                                            UT_REGISTERS_ADDRESS,
                                            UT_REGISTERS_NB,
                                            &mut tab_rp_registers).unwrap();

    print!("4/5 modbus_write_and_read_registers: ");
    assert_true!(rc == UT_REGISTERS_NB, "FAILED (nb_points {} != {})", rc, UT_REGISTERS_NB);

    assert_true!(tab_rp_registers[0] == UT_REGISTERS_TAB[0],
            "FAILED ({:?} != {:?})", tab_rp_registers[0], UT_REGISTERS_TAB[0]);

    for i in 1..UT_REGISTERS_NB as usize {
        assert_true!(tab_rp_registers[i] == 0,
            "FAILED ({:0X} != {:0X})", tab_rp_registers[i], 0);
    }

    // End of many registers


    // INPUT REGISTERS
    let rc = modbus.read_input_registers(UT_INPUT_REGISTERS_ADDRESS,
        UT_INPUT_REGISTERS_NB,
        &mut tab_rp_registers).unwrap();

    print!("1/1 modbus_read_input_registers: ");
    assert_true!(rc == UT_INPUT_REGISTERS_NB, "FAILED (nb_points {})", rc);

    for i in 0..UT_INPUT_REGISTERS_NB as usize {
         assert_true!(tab_rp_registers[i] == UT_INPUT_REGISTERS_TAB[i],
                     "FAILED ({:0X} != {:0X})",
                     tab_rp_registers[i], UT_INPUT_REGISTERS_TAB[i]);
    }

    // MASKS
    print!("1/1 Write mask: ");
    let _rc = modbus.write_register(UT_REGISTERS_ADDRESS, 0x12).unwrap();
    let rc = modbus.mask_write_register(UT_REGISTERS_ADDRESS, 0xF2, 0x25);
    assert_true!(rc.is_ok(), "FAILED ({:?} == -1)", rc);
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS, 1, &mut tab_rp_registers).unwrap();
    assert_true!(tab_rp_registers[0] == 0x17,
                "FAILED ({:0X} != {:0X})",
                tab_rp_registers[0], 0x17);

    print!("\nTEST FLOATS");
    // FLOAT
    print!("1/4 Set/get float ABCD: ");
    set_float_abcd(UT_REAL, &mut tab_rp_registers);
    assert_true!(equal_dword(&tab_rp_registers, UT_IREAL_ABCD), "FAILED Set float ABCD");
    let real = get_float_abcd(&tab_rp_registers[0..2]);
    assert_true!(real == UT_REAL, "FAILED ({} != {})", real, UT_REAL);

    print!("2/4 Set/get float DCBA: ");
    set_float_dcba(UT_REAL, &mut tab_rp_registers);
    assert_true!(equal_dword(&tab_rp_registers, UT_IREAL_DCBA), "FAILED Set float DCBA");
    let real = get_float_dcba(&tab_rp_registers[0..2]);
    assert_true!(real == UT_REAL, "FAILED ({} != {})", real, UT_REAL);

    print!("3/4 Set/get float BADC: ");
    set_float_badc(UT_REAL, &mut tab_rp_registers);
    assert_true!(equal_dword(&tab_rp_registers, UT_IREAL_BADC), "FAILED Set float BADC");
    let real = get_float_badc(&tab_rp_registers[0..2]);
    assert_true!(real == UT_REAL, "FAILED ({} != {})", real, UT_REAL);

    print!("4/4 Set/get float CDAB: ");
    set_float_cdab(UT_REAL, &mut tab_rp_registers);
    assert_true!(equal_dword(&tab_rp_registers, UT_IREAL_CDAB), "FAILED Set float CDAB");
    let real = get_float_cdab(&tab_rp_registers[0..2]);
    assert_true!(real == UT_REAL, "FAILED ({} != {})", real, UT_REAL);

    print!("\nAt this point, error messages doesn't mean the test has failed");

    // ILLEGAL DATA ADDRESS
    print!("\nTEST ILLEGAL DATA ADDRESS:");

    /* The mapping begins at the defined addresses and ends at address +
    * nb_points so these addresses are not valid. */

    let rc = modbus.read_bits(0, 1, &mut tab_rp_bits);
    print!("* read_bits (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_bits(UT_BITS_ADDRESS, UT_BITS_NB + 1, &mut tab_rp_bits);
    print!("* read_bits (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_bits(0, 1, &mut tab_rp_bits);
    print!("* read_input_bits (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_bits(UT_INPUT_BITS_ADDRESS,
                                UT_INPUT_BITS_NB + 1, &mut tab_rp_bits);
    print!("* read_input_bits (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_registers(0, 1, &mut tab_rp_registers);
    print!("* read_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                               UT_REGISTERS_NB_MAX + 1, &mut tab_rp_registers);
    print!("* read_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_registers(0, 1, &mut tab_rp_registers);
    print!("* read_input_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.read_input_registers(UT_INPUT_REGISTERS_ADDRESS,
                                     UT_INPUT_REGISTERS_NB + 1,
                                     &mut tab_rp_registers);
    print!("* read_input_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bit(0, true);
    print!("* write_bit (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bit(UT_BITS_ADDRESS + UT_BITS_NB, true);
    print!("* write_bit (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bits(0, 1, &mut tab_rp_bits);
    print!("* write_coils (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_bits(UT_BITS_ADDRESS + UT_BITS_NB,
                           UT_BITS_NB, &mut tab_rp_bits);
    print!("* write_coils (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_register(0, tab_rp_registers[0]);
    print!("* write_register (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_register(UT_REGISTERS_ADDRESS + UT_REGISTERS_NB_MAX,
                                tab_rp_registers[0]);
    print!("* write_register (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_registers(0, 1, &mut tab_rp_registers);
    print!("* write_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_registers(UT_REGISTERS_ADDRESS + UT_REGISTERS_NB_MAX,
                                UT_REGISTERS_NB, &mut tab_rp_registers);
    print!("* write_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.mask_write_register(0, 0xF2, 0x25);
    print!("* mask_write_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.mask_write_register(UT_REGISTERS_ADDRESS + UT_REGISTERS_NB_MAX,
                                    0xF2, 0x25);
    print!("* mask_write_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_and_read_registers(0, 1, &tab_rp_registers.clone(), 0, 1, &mut tab_rp_registers);
    print!("* write_and_read_registers (0): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    let rc = modbus.write_and_read_registers(UT_REGISTERS_ADDRESS + UT_REGISTERS_NB_MAX,
                                         UT_REGISTERS_NB, &tab_rp_registers.clone(),
                                         UT_REGISTERS_ADDRESS + UT_REGISTERS_NB_MAX,
                                         UT_REGISTERS_NB, &mut tab_rp_registers);
    print!("* write_and_read_registers (max): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Illegal data address", "");

    // TOO MANY DATA
    print!("\nTOO MANY DATA ERROR:\n");

    let rc = modbus.read_bits(UT_BITS_ADDRESS,
                          Modbus::MAX_READ_BITS as u16 + 1, &mut tab_rp_bits);
    print!("* read_bits: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_input_bits(UT_INPUT_BITS_ADDRESS,
                                Modbus::MAX_READ_BITS as u16 + 1, &mut tab_rp_bits);
    print!("* read_input_bits: ");
    // assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                               Modbus::MAX_READ_REGISTERS as u16 + 1,
                               &mut tab_rp_registers);
    print!("* read_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.read_input_registers(UT_INPUT_REGISTERS_ADDRESS,
                                     Modbus::MAX_READ_REGISTERS as u16 + 1,
                                     &mut tab_rp_registers);
    print!("* read_input_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.write_bits(UT_BITS_ADDRESS,
                           Modbus::MAX_WRITE_BITS as u16 + 1, &mut tab_rp_bits);
    print!("* write_bits: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    let rc = modbus.write_registers(UT_REGISTERS_ADDRESS,
                                Modbus::MAX_WRITE_REGISTERS as u16 + 1,
                                &mut tab_rp_registers);
    print!("* write_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Too many data", "");

    // SLAVE REPLY
    let old_slave = modbus.get_slave().unwrap();

    modbus.set_slave(INVALID_SERVER_ID);
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                                UT_REGISTERS_NB, &mut tab_rp_registers);

    if backend == Backend::RTU {
        const RAW_REQ_LENGTH: i32 = 6;
        let mut raw_req = vec![INVALID_SERVER_ID, 0x03, 0x00, 0x01, 0x01, 0x01];
        /* Too many points */
        let mut raw_invalid_req = vec![INVALID_SERVER_ID, 0x03, 0x00, 0x01, 0xFF, 0xFF];
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

        /* Send an INVALID request for another slave */
        modbus.send_raw_request(&mut raw_invalid_req, RAW_REQ_LENGTH * std::mem::size_of::<u8>() as i32);
        let rc = modbus.receive_confirmation(&mut rsp);

        print!("1-C/3 No response from slave {} with invalid request: ",
               INVALID_SERVER_ID);
        assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Timeout", "");

        let rc = modbus.set_slave(Modbus::BROADCAST_ADDRESS);
        assert_true!(rc.is_ok(), "Invalid broadcast address");

        let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                                   UT_REGISTERS_NB, &mut tab_rp_registers);
        print!("2/3 No reply after a broadcast query: ");
        assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Timeout", "");
    } else {
        /* Response in TCP mode */
        print!("1/3 Response from slave {}: ", INVALID_SERVER_ID);
        assert_true!(rc.unwrap() == UT_REGISTERS_NB, "");

        let rc = modbus.set_slave(Modbus::BROADCAST_ADDRESS);
        assert_true!(rc.is_ok(), "Invalid broacast address");

        let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                                   UT_REGISTERS_NB, &mut tab_rp_registers);
        print!("2/3 Reply after a query with unit id == 0: ");
        assert_true!(rc.unwrap() == UT_REGISTERS_NB, "");
    }


    /* Restore slave */
    modbus.set_slave(old_slave);

    print!("3/3 Response with an invalid TID or slave: ");
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_INVALID_TID_OR_SLAVE,
                               1, &mut tab_rp_registers);
    assert_true!(rc.is_err(), "");

    print!("1/2 Report slave ID truncated: \n");
    /* Set a marker to ensure limit is respected */
    tab_rp_bits[NB_REPORT_SLAVE_ID - 1] = 42;
    let rc = modbus.report_slave_id(NB_REPORT_SLAVE_ID - 1, &mut tab_rp_bits).unwrap();
    /* Return the size required (response size) but respects the defined limit */
    assert_true!(rc == NB_REPORT_SLAVE_ID as u16 &&
                tab_rp_bits[NB_REPORT_SLAVE_ID - 1] == 42,
                "Return is rc {} ({}) and marker is {} (42)",
                rc, NB_REPORT_SLAVE_ID, &mut tab_rp_bits[NB_REPORT_SLAVE_ID - 1]);

    print!("2/2 Report slave ID: \n");
    /* tab_rp_bits is used to store bytes */
    let rc = modbus.report_slave_id(NB_REPORT_SLAVE_ID, &mut tab_rp_bits).unwrap();
    assert_true!(rc == NB_REPORT_SLAVE_ID as u16, "");

    /* Slave ID is an arbitraty number for libmodbus */
    assert_true!(rc > 0, "");

    /* Run status indicator is ON */
    assert_true!(rc > 1 && tab_rp_bits[1] == 0xFF, "");

    /* Print additional data as string */
    if rc > 2 {
        print!("Additional data: ");
        for i in 2..rc as usize {
            print!("{}", tab_rp_bits[i] as char);
        }
        println!();
    }

    /* Save original timeout */
    let old_response_timeout = modbus.get_response_timeout().unwrap();
    let old_byte_timeout = modbus.get_byte_timeout().unwrap();

    let rc = modbus.set_response_timeout(Default::default());
    print!("1/6 Invalid response timeout (zero): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    let rc = modbus.set_response_timeout( Timeout::new_usec( 1_000_000 ) );
    print!("2/6 Invalid response timeout (too large us): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    let rc = modbus.set_byte_timeout( Timeout::new_usec( 1_000_000 ) );
    print!("3/6 Invalid byte timeout (too large us): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    modbus.set_response_timeout(Timeout::new_usec(1));
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS, UT_REGISTERS_NB, &mut tab_rp_registers);
    print!("4/6 1us response timeout: ");
    if rc.is_err() && rc.unwrap_err().to_string() == "Connection timed out (os error 110)" {
        println!("OK");
    } else {
        println!("FAILED (can fail on some platforms)");
    }

    /* A wait and flush operation is done by the error recovery code of
    * libmodbus but after a sleep of current response timeout
    * so 0 can be too short!
    */
    sleep(Duration::new(old_response_timeout.sec as u64, old_response_timeout.usec * 1_000));
    modbus.flush();

    /* Trigger a special behaviour on server to wait for 0.5 second before
    * replying whereas allowed timeout is 0.2 second */
    modbus.set_response_timeout(Timeout::new(0, 200));
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_SLEEP_500_MS, 1, &mut tab_rp_registers);
    print!("5/6 Too short response timeout (0.2s < 0.5s): ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Connection timed out (os error 110)", "");

    /* Wait for reply (0.2 + 0.4 > 0.5 s) and flush before continue */
    sleep(Duration::new(0, 400_000_000));
    modbus.flush();

    modbus.set_response_timeout(Timeout::new(0, 600_000));
    println!("{:?}", modbus.get_response_timeout());
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_SLEEP_500_MS, 1, &mut tab_rp_registers);
    print!("6/6 Adequate response timeout (0.6s > 0.5s): ");
    assert_true!(rc.is_ok() && rc.unwrap() == 1, "");

    /* Disable the byte timeout.
    The full response must be available in the 600ms interval */
    modbus.set_byte_timeout(Timeout::new(0, 0));
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_SLEEP_500_MS, 1, &mut tab_rp_registers);
    print!("7/7 Disable byte timeout: ");
    assert_true!(rc.is_ok(), "");

    /* Restore original response timeout */
    modbus.set_response_timeout(old_response_timeout);

    if backend == Backend::TCP {
        /* The test server is only able to test byte timeouts with the TCP
         * backend */

        /* Timeout of 3ms between bytes */
        modbus.set_byte_timeout(Timeout::new(0, 3000));
        let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_BYTE_SLEEP_5_MS,
                                   1, &mut tab_rp_registers);
        print!("1/2 Too small byte timeout (3ms < 5ms): ");
        assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Connection timed out (os error 110)", "");

        /* Wait remaing bytes before flushing */
        sleep(Duration::from_millis(11 * 5));
        modbus.flush();

        /* Timeout of 7ms between bytes */
        modbus.set_byte_timeout(Timeout::new(0, 7000));
        let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_BYTE_SLEEP_5_MS,
                                   1, &mut tab_rp_registers);
        print!("2/2 Adapted byte timeout (7ms > 5ms): ");
        assert_true!(rc.is_ok(), "");
    }

    /* Restore original byte timeout */
    modbus.set_byte_timeout(old_byte_timeout);

    /** BAD RESPONSE **/
    println!("\nTEST BAD RESPONSE ERROR:");

    /* Allocate only the required space */
    let mut tab_rp_registers_bad = vec![0u16; (UT_REGISTERS_NB_SPECIAL * std::mem::size_of::<u16>() as u16) as usize ];

    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS,
                               UT_REGISTERS_NB_SPECIAL, &mut tab_rp_registers_bad);
    print!("* modbus.read_registers: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Invalid data", "");

    /** MANUAL EXCEPTION **/
    println!("\nTEST MANUAL EXCEPTION:");
    let rc = modbus.read_registers(UT_REGISTERS_ADDRESS_SPECIAL,
                               UT_REGISTERS_NB, &mut tab_rp_registers);

    print!("* modbus.read_registers at special address: ");
    assert_true!(rc.is_err() && rc.unwrap_err().to_string() == "Slave device or server is busy", "");

    /* Run a few tests to challenge the server code */
    if test_server(&mut modbus, backend).is_err() {
        std::process::exit(-1)
    }

    modbus.close();
    modbus.free();

    /* Test init functions */
    println!("\nTEST INVALID INITIALIZATION:");
    let mut modbus = Modbus::new_rtu("", 1, 'A', 0, 0);
    assert_true!(modbus.is_err() && modbus.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    let mut modbus = Modbus::new_rtu("/dev/dummy", 0, 'A', 0, 0);
    assert_true!(modbus.is_err() && modbus.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    let mut modbus = Modbus::new_tcp_pi("", "");
    assert_true!(modbus.is_err() && modbus.unwrap_err().to_string() == "Invalid argument (os error 22)", "");

    print!("\nALL TESTS PASS WITH SUCCESS.\n");
    let success = true;

    if success { Ok(()) } else { Err(ErrorKind::UnitTestClientFailure.into()) }
}

fn test_server(modbus: &mut Modbus, backend: Backend) -> Result<()> {
    /* Read requests */
    const READ_RAW_REQ_LEN: i32 = 6;
    let slave = match backend {
        Backend::RTU => SERVER_ID,
        _ => Modbus::TCP_SLAVE,
    };
    let mut read_raw_req: Vec<u8> = vec![
        slave,
        /* function, address, 5 values */
        FunctionCode::ReadHoldingRegisters as u8,
        (UT_REGISTERS_ADDRESS >> 8) as u8, (UT_REGISTERS_ADDRESS & 0xFF) as u8,
        0x0, 0x05
    ];
    /* Write and read registers request */
    const RW_RAW_REQ_LEN: i32 = 13;
    let mut rw_raw_req = vec![
        slave,
        /* function, addr to read, nb to read */
        FunctionCode::WriteAndReadRegisters as u8,
        /* Read */
        (UT_REGISTERS_ADDRESS >> 8) as u8, (UT_REGISTERS_ADDRESS & 0xFF) as u8,
        ((Modbus::MAX_WR_READ_REGISTERS + 1) >> 8) as u8,
        ((Modbus::MAX_WR_READ_REGISTERS + 1) & 0xFF) as u8,
        /* Write */
        0, 0,
        0, 1,
        /* Write byte count */
        1 * 2,
        /* One data to write... */
        0x12, 0x34
    ];
    const WRITE_RAW_REQ_LEN: i32 = 13;
    let mut write_raw_req = vec![
        slave,
        /* function will be set in the loop */
        FunctionCode::WriteMultipleRegisters as u8,
        /* Address */
        (UT_REGISTERS_ADDRESS >> 8) as u8, (UT_REGISTERS_ADDRESS & 0xFF) as u8,
        /* 3 values, 6 bytes */
        0x00, 0x03, 0x06,
        /* Dummy data to write */
        0x02, 0x2B, 0x00, 0x01, 0x00, 0x64
    ];
    const INVALID_FC: u8 = 0x42;
    const INVALID_FC_REQ_LEN: i32 = 6;
    let mut invalid_fc_raw_req: Vec<u8> = vec![
        slave, 0x42, 0x00, 0x00, 0x00, 0x00
    ];

    // int req_length;
    let mut rsp: Vec<u8> = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];
    let tab_read_function = vec![
        FunctionCode::ReadCoils,
        FunctionCode::ReadDiscreteInputs,
        FunctionCode::ReadHoldingRegisters,
        FunctionCode::ReadInputRegisters,
    ];
    let tab_read_nb_max = vec![
        Modbus::MAX_READ_BITS + 1,
        Modbus::MAX_READ_BITS + 1,
        Modbus::MAX_READ_REGISTERS + 1,
        Modbus::MAX_READ_REGISTERS + 1
    ];

    let (backend_length, backend_offset) = match backend {
        Backend::RTU => (3, 1),
        _ => (7, 7),
    };

    print!("\nTEST RAW REQUESTS:\n");

    /* This requests can generate flushes server side so we need a higher
     * response timeout than the server. The server uses the defined response
     * timeout to sleep before flushing.
     * The old timeouts are restored at the end.
     */
    let old_response_timeout = modbus.get_response_timeout().unwrap();
    modbus.set_response_timeout(Timeout::new(0, 600000));

    let req_length = modbus.send_raw_request(&mut read_raw_req.clone(), READ_RAW_REQ_LEN).unwrap();
    print!("* modbus_send_raw_request: ");
    assert_true!(req_length == (backend_length + 5), "FAILED ({})\n", req_length);

    print!("* modbus_receive_confirmation: ");
    let rc = modbus.receive_confirmation(&mut rsp.clone()).unwrap();
    assert_true!(rc == (backend_length + 12), "FAILED ({})\n", rc);

    /* Try to read more values than a response could hold for all data
       types. */
    for i in 0..4 {
        let rc = send_crafted_request(modbus, tab_read_function[i],
                                  &mut read_raw_req, READ_RAW_REQ_LEN,
                                  tab_read_nb_max[i] as u16, 0,
                                  backend_length, backend_offset);
        if rc.is_err() {
            modbus.set_response_timeout(old_response_timeout);
            std::process::exit(-1)
        }
    }

    /* Modbus write and read multiple registers */
    let rc = send_crafted_request(modbus, FunctionCode::WriteAndReadRegisters,
                              &mut rw_raw_req, RW_RAW_REQ_LEN,
                              (Modbus::MAX_WR_READ_REGISTERS + 1) as u16, 0,
                              backend_length, backend_offset);
    if rc.is_err() {
        modbus.set_response_timeout(old_response_timeout);
        std::process::exit(-1)
    }

    /* Modbus write multiple registers with large number of values but a set a
       small number of bytes in requests (not nb * 2 as usual). */
    let rc = send_crafted_request(modbus, FunctionCode::WriteMultipleRegisters,
                              &mut write_raw_req, WRITE_RAW_REQ_LEN,
                              (Modbus::MAX_WRITE_REGISTERS + 1) as u16, 6,
                              backend_length, backend_offset);
    if rc.is_err() {
        modbus.set_response_timeout(old_response_timeout);
        std::process::exit(-1)
    }

    let rc = send_crafted_request(modbus, FunctionCode::WriteMultipleCoils,
                              &mut write_raw_req, WRITE_RAW_REQ_LEN,
                              (Modbus::MAX_WRITE_BITS + 1) as u16, 6,
                              backend_length, backend_offset);
    if rc.is_err() {
        modbus.set_response_timeout(old_response_timeout);
        std::process::exit(-1)
    }

    /* Test invalid function code */
    modbus.send_raw_request(&mut invalid_fc_raw_req, INVALID_FC_REQ_LEN * std::mem::size_of::<u8>() as i32);
    let rc = modbus.receive_confirmation(&mut rsp).unwrap();
    print!("Return an exception on unknown function code: ");
    assert_true!(rc == (backend_length + EXCEPTION_RC) && rsp[backend_offset as usize] == (0x80 + INVALID_FC), "");

    modbus.set_response_timeout(old_response_timeout);

    // // Err(ErrorKind::UnitTestClientFailure.into())
    Ok(())
}

fn send_crafted_request(modbus: &mut Modbus, function: FunctionCode,
    mut req: &mut [u8], req_len: i32,
    max_value: u16, bytes: u16,
    backend_length: u16, backend_offset: u16) -> Result<()>
{
    let mut rsp = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];

    for j in 0..2 {
        req[1] = function as u8;
        if j == 0 {
            /* Try to read or write zero values on first iteration */
            req[4] = 0x00;
            req[5] = 0x00;
            if bytes > 0 {
                /* Write query */
                req[6] = 0x00;
            }
        } else {
            /* Try to read or write max values + 1 on second iteration */
            req[4] = (max_value >> 8) as u8 & 0xFF;
            req[5] = max_value as u8 & 0xFF;
            if bytes > 0 {
                /* Write query (nb values * 2 to convert in bytes for registers) */
                req[6] = bytes as u8;
            }
        }

        modbus.send_raw_request(&mut req, req_len * std::mem::size_of::<u8>() as i32);
        if j == 0 {
            print!("* try function 0x{:X}: {} 0 values: ", function as u8, if bytes > 0 { "write" } else { "read" });
        } else {
            print!("* try function 0x{:X}: {} {} values: ", function as u8, if bytes > 0 { "write" } else { "read" }, max_value);
        }
        let rc = modbus.receive_confirmation(&mut rsp).unwrap();
        assert_true!(rc == (backend_length + EXCEPTION_RC) &&
                    rsp[backend_offset as usize] == (0x80 + function as u8) &&
                    rsp[backend_offset as usize + 1] == Exception::IllegalDataValue as u8, "");
    }

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
