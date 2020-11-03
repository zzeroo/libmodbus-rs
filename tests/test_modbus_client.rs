use libmodbus::{Modbus, ModbusClient, ModbusServer, ModbusMapping, FunctionCode, ModbusTCP};
use std::thread;
use std::time::Duration;

fn start_server(port: i32) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut modbus = Modbus::new_tcp("127.0.0.1", port).expect("Could not create TCP Server context");
        let mut socket = modbus.tcp_listen(1).expect("Could not listen to TCP socket");
        modbus.tcp_accept(&mut socket).expect("Could not accept connection");

        let mb_mapping = ModbusMapping::new(Modbus::MAX_READ_BITS,
                                            Modbus::MAX_READ_BITS,
                                            Modbus::MAX_READ_REGISTERS,
                                            Modbus::MAX_READ_REGISTERS).expect("Failed to allocate the mapping");

            loop {
                let mut query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];

                match modbus.receive(&mut query) {
                    Ok(rc) => modbus.reply(&query, rc, &mb_mapping),
                    Err(_err) => break,
                }.expect("Could not receive");
            }
    })
}


#[test]
fn read_bits() {
    let port = 1502;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            let mut dest = vec![0u8; 100];
            client.connect().expect("could not connect");
            assert!(client.read_bits(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn read_input_bits() {
    let port = 1503;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            let mut dest = vec![0u8; 100];
            client.connect().expect("could not connect");
            assert!(client.read_input_bits(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn read_registers() {
    let port = 1504;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            let mut dest = vec![0u16; 100];
            client.connect().expect("could not connect");
            assert!(client.read_registers(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn read_input_registers() {
    let port = 1505;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            let mut dest = vec![0u16; 100];
            client.connect().expect("could not connect");
            assert!(client.read_input_registers(0, 1, &mut dest).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn report_slave_id() {
    let port = 1506;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            let mut bytes = vec![0u8; Modbus::MAX_PDU_LENGTH];
            client.connect().expect("could not connect");
            // println!("{:?}", str::from_utf8(&client.report_slave_id[2..])) # => Ok("LMB3.1.4")
            assert!(client.report_slave_id(Modbus::MAX_PDU_LENGTH, &mut bytes).is_ok());
            // assert_eq!(client.report_slave_id().unwrap(), vec![180, 255, 76, 77, 66, 51, 46, 49, 46, 52]);
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn write_bit() {
    let port = 1507;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            assert!(client.write_bit(0, true).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn write_bits() {
    let port = 1508;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let src = vec![1u8];
            assert_eq!(client.write_bits(0, 1, &src).unwrap(), 1);
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn write_register() {
    let port = 1509;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let address: u16 = 1;
            let value = u16::max_value();
            assert!(client.write_register(address, value).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn write_registers() {
    let port = 1510;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let address = 1;
            let tab_bytes = vec![0u16];
            assert_eq!(client.write_registers(address, 1, &tab_bytes).unwrap(), 1);
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn write_and_read_registers() {
    let port = 1511;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn send_raw_request() {
    let port = 1512;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::ReadHoldingRegisters as u8, 0x00, 0x01, 0x0, 0x05];
            let mut response = vec![0u8; Modbus::MAX_ADU_LENGTH];
            assert_eq!(client.send_raw_request(&mut raw_request, 12).unwrap(), 18);
            assert!(client.receive_confirmation(&mut response).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}

#[test]
fn receive_confirmation() {
    let port = 1513;
    // Start modbus server
    let server_thread = start_server(port);
    thread::sleep(Duration::from_millis(200));

    // connect client
    match Modbus::new_tcp("127.0.0.1", port) {
        Ok(client) => {
            client.connect().expect("could not connect");
            let mut raw_request: Vec<u8> = vec![0xFF, FunctionCode::ReadHoldingRegisters as u8, 0x00, 0x01, 0x0, 0x05];
            let mut response = vec![0u8; Modbus::MAX_ADU_LENGTH];
            assert_eq!(client.send_raw_request(&mut raw_request, 12).unwrap(), 18);
            assert!(client.receive_confirmation(&mut response).is_ok());
        },
        _ => panic!("could not connect"),
    }

    let _ = server_thread.join();
}
