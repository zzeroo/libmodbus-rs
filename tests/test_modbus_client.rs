extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusClient, ModbusServer, ModbusMapping, FunctionCode, ModbusTCP};
use std::thread;
use std::time::Duration;

fn start_server() {
    thread::spawn(move || {
        match Modbus::new_tcp("127.0.0.1", 1502) {
            Ok(mut server) => {
                let mapping = ModbusMapping::new(10, 10, 10, 10).expect("could not create modbus mapping");
                let mut socket = server.tcp_listen(1).expect("could not listen");
                server.tcp_accept(&mut socket).expect("unable to accept TCP socket");
                let mut query = vec![0; Modbus::MAX_ADU_LENGTH];
                match server.receive(&mut query) {
                    Ok(request) => {
                        server.reply(&mut query, 1, &mapping).expect(&format!("could not reply requeset: {}", request));
                        Ok(())
                    },
                    Err(err) => Err(err),
                }
            },
            Err(err) => panic!("Could not create server: {}", err),
        }
    });
    // give server some time to come up
    thread::sleep(Duration::from_millis(100));
}


// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn read_bits() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut dest = vec![0u8; 100];
            client.connect().expect("could not connect");
            assert!(client.read_bits(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn read_bits_too_many() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut dest = vec![0u8; 100];
            client.connect().expect("could not connect");
            assert!(client.read_bits(0, 101, &mut dest).is_err()); // => ErrorKind::TooManyDataBits
        },
        _ => panic!("could not connect"),
    }
}

// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn read_input_bits() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut dest = vec![0u8; 100];
            client.connect().expect("could not connect");
            assert!(client.read_input_bits(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn read_registers() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut dest = vec![0u16; 100];
            client.connect().expect("could not connect");
            assert!(client.read_registers(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn read_input_registers() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut dest = vec![0u16; 100];
            client.connect().expect("could not connect");
            assert!(client.read_input_registers(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

// FIXME: Find way to stop the server between the test
#[test]
#[ignore]
fn report_slave_id() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            let mut bytes = vec![0u8; Modbus::MAX_PDU_LENGTH];
            client.connect().expect("could not connect");
            // println!("{:?}", str::from_utf8(&client.report_slave_id[2..])) # => Ok("LMB3.1.4")
            assert!(client.report_slave_id(Modbus::MAX_PDU_LENGTH, &mut bytes).is_ok());
            // assert_eq!(client.report_slave_id().unwrap(), vec![180, 255, 76, 77, 66, 51, 46, 49, 46, 52]);
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn write_bit() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            assert!(client.write_bit(0, true).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn write_bits() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let src = vec![1u8];
            assert_eq!(client.write_bits(0, 1, &src).unwrap(), 1);
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn write_register() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let address: u16 = 1;
            let value = u16::max_value();
            assert!(client.write_register(address, value).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn write_registers() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let address = 1;
            let tab_bytes = vec![0u16];
            assert_eq!(client.write_registers(address, 1, &tab_bytes).unwrap(), 1);
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn write_and_read_registers() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn send_raw_request() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::ReadHoldingRegisters as u8, 0x00, 0x01, 0x0, 0x05];
            assert_eq!(client.send_raw_request(&mut raw_request).unwrap(), 12);
            assert_eq!(client.receive_confirmation().unwrap(),
                       vec![0, 0, 0, 0, 0, 13, 255, 3, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn receive_confirmation() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::ReadHoldingRegisters as u8, 0x00, 0x01, 0x0, 0x05];
            assert_eq!(client.send_raw_request(&mut raw_request).unwrap(), 12);
            assert_eq!(client.receive_confirmation().unwrap(),
                       vec![0, 0, 0, 0, 0, 13, 255, 3, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]);
        },
        _ => panic!("could not connect"),
    }
}
