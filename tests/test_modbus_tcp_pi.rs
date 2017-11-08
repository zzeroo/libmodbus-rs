extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusTCPPI};


#[test]
fn new_tcp_pi() {
    assert!(Modbus::new_tcp_pi("::1", "1502").is_ok());
}

#[test]
#[ignore]
fn tcp_pi_accept() {
    let mut server = Modbus::new_tcp_pi("::1", "1502").unwrap();
    match server.tcp_pi_listen(1) {
        Ok(mut socket) => assert_eq!(server.tcp_pi_accept(&mut socket).unwrap(), 1),
        _ => panic!("could not listen to socket"),
    }
}

#[test]
#[ignore]
fn tcp_pi_listen() {
    let mut server = Modbus::new_tcp_pi("::1", "1502").unwrap();
    let client = Modbus::new_tcp_pi("::1", "1502").unwrap();
    match server.tcp_pi_listen(1) {
        Ok(mut _socket) => assert!(client.connect().is_ok()),
        _ => panic!("could not listen to socket"),
    }
}
