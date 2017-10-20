extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer, ModbusTCP};


#[test]
#[ignore]
fn receive() {
    let mut query = vec![0; Modbus::MAX_ADU_LENGTH as usize];
    // create server
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(mut server) => {
            let mut socket = server.tcp_listen(1).expect("could not listen");
            server.tcp_accept(&mut socket).expect("unable to accept TCP socket");
        },
        Err(err) => panic!("Could not create server: {}", err),
    }

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            assert!(client.receive(&mut query).is_ok());
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn reply() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // &self, request: &[u8], request_len: i32, modbus_mapping: &ModbusMapping) -> Result<i32>;
}
