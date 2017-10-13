extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusTCP};


#[test]
fn new_tcp() {
    assert!(Modbus::new_tcp("127.0.0.1", 1502).is_ok());
}

#[test]
#[ignore]
fn tcp_accept() {
    let mut server = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    match server.tcp_listen(1) {
        Ok(mut socket) => assert_eq!(server.tcp_accept(&mut socket).unwrap(), 1),
        _ => panic!("could not listen to socket"),
    }
}

#[test]
fn tcp_listen() {
    let mut server = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    let client = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    match server.tcp_listen(1) {
        Ok(mut _socket) => assert!(client.connect().is_ok()),
        _ => panic!("could not listen to socket"),
    }
}
