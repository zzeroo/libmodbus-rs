extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusTCP};


#[test]
fn new_tcp() {
    assert!(Modbus::new_tcp("127.0.0.1", 1502).is_ok());
}

#[test]
#[ignore]
fn tcp_accept() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    match modbus.tcp_listen(1) {
        Ok(mut socket) => assert_eq!(modbus.tcp_accept(&mut socket).unwrap(), 1),
        _ => panic!("could not listen to socket"),
    }
}

#[test]
#[ignore]
fn tcp_listen() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(modbus.tcp_listen(1).is_ok());
}
