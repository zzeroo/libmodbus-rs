extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusTCPPI};


#[test]
fn new_tcp_pi() {
    assert!(Modbus::new_tcp_pi("::1", "1502").is_ok());
    // ip: &str, port: i32) -> Result<Modbus>;
}

#[test]
fn tcp_accept_pi() {
    let modbus = Modbus::new_tcp_pi("::1", "1502").unwrap();
    // socket: &mut i32) -> Result<i32>;
}


#[test]
fn tcp_listen_pi() {
    let modbus = Modbus::new_tcp_pi("::1", "1502").unwrap();
    // assert!(modbus.tcp_listen_pi(1).is_ok());
}
