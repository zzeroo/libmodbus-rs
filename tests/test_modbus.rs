extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, Timeout, ModbusTCP};


#[test]
fn connect() {
    // create server
    match Modbus::new_tcp("127.0.0.1", 1502) {
            Ok(mut server) => server.tcp_listen(1),
            Err(err) => panic!("Could not create server: {}", err),
        }
        .unwrap();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(modbus) => assert!(modbus.connect().is_ok()),
        _ => panic!("could not connect"),
    }
}

#[test]
fn flush() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(modbus.flush().is_ok());
}

#[test]
fn set_slave() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(modbus.set_slave(0).is_ok());
    assert!(modbus.set_slave(1).is_ok());
    // FIXME: rust silent overflow, allows invalid slave ID
    // assert!(modbus.set_slave(266).is_ok());
}

#[test]
fn set_debug() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(modbus.set_debug(true).is_ok());
}

#[test]
fn get_byte_timeout() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(modbus.get_byte_timeout().unwrap(),
               libmodbus_rs::Timeout {
                   sec: 0,
                   usec: 500000,
               });
}

#[test]
fn set_byte_timeout() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(modbus.get_byte_timeout().unwrap(),
               Timeout {
                   sec: 0,
                   usec: 500000,
               });

    let timeout = Timeout {
        sec: 1,
        usec: 500000,
    };
    assert!(modbus.set_byte_timeout(timeout).is_ok());
    assert_eq!(modbus.get_byte_timeout().unwrap(),
               Timeout {
                   sec: 1,
                   usec: 500000,
               });
}

#[test]
fn get_response_timeout() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(modbus.get_response_timeout().unwrap(),
               Timeout {
                   sec: 0,
                   usec: 500000,
               });
}

#[test]
fn set_response_timeout() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(modbus.get_response_timeout().unwrap(),
               Timeout {
                   sec: 0,
                   usec: 500000,
               });

    let timeout = Timeout {
        sec: 1,
        usec: 500000,
    };
    assert!(modbus.set_response_timeout(timeout).is_ok());
    assert_eq!(modbus.get_response_timeout().unwrap(),
               Timeout {
                   sec: 1,
                   usec: 500000,
               });
}

#[test]
#[ignore]
fn set_error_recovery() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
}

#[test]
#[ignore]

fn set_socket() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // (&mut self, socket: i32) -> Result<i32>
}

#[test]
#[ignore]
fn get_socket() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // (&self) -> Result<i32>
}

#[test]
#[ignore]
fn get_header_length() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // (&self) -> i32
}

#[test]
#[ignore]
fn close() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // (&self)
}

#[test]
#[ignore]
fn free() {
    let _modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // (&mut self)
}
