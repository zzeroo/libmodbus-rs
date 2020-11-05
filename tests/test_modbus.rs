use libmodbus::{Modbus, ModbusTCP, Timeout};

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
#[ignore]
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
    assert_eq!(
        modbus.get_byte_timeout().unwrap(),
        Timeout {
            sec: 0,
            usec: 500000,
        }
    );
}

#[test]
fn set_byte_timeout() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(
        modbus.get_byte_timeout().unwrap(),
        Timeout {
            sec: 0,
            usec: 500000,
        }
    );

    let timeout = Timeout {
        sec: 1,
        usec: 500000,
    };
    assert!(modbus.set_byte_timeout(timeout).is_ok());
    assert_eq!(
        modbus.get_byte_timeout().unwrap(),
        Timeout {
            sec: 1,
            usec: 500000,
        }
    );
}

#[test]
fn get_response_timeout() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(
        modbus.get_response_timeout().unwrap(),
        Timeout {
            sec: 0,
            usec: 500000,
        }
    );
}

#[test]
fn set_response_timeout() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(
        modbus.get_response_timeout().unwrap(),
        Timeout {
            sec: 0,
            usec: 500000,
        }
    );

    let timeout = Timeout {
        sec: 1,
        usec: 500000,
    };
    assert!(modbus.set_response_timeout(timeout).is_ok());
    assert_eq!(
        modbus.get_response_timeout().unwrap(),
        Timeout {
            sec: 1,
            usec: 500000,
        }
    );
}

#[test]
fn set_error_recovery() {
    use libmodbus::ErrorRecoveryMode;
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();

    assert!(modbus.set_error_recovery(None).is_ok());
    assert!(modbus
        .set_error_recovery(Some(&[ErrorRecoveryMode::Link]))
        .is_ok());
    assert!(modbus
        .set_error_recovery(Some(&[ErrorRecoveryMode::Protocol]))
        .is_ok());
    assert!(modbus
        .set_error_recovery(Some(&[
            ErrorRecoveryMode::Link,
            ErrorRecoveryMode::Protocol
        ]))
        .is_ok());
}

#[test]
fn set_socket() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert!(modbus.set_socket(1337).is_ok());
}

#[test]
fn get_socket() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.set_socket(1337).expect("could not set socket");
    assert_eq!(modbus.get_socket().unwrap(), 1337);
}

#[test]
fn get_header_length() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    assert_eq!(modbus.get_header_length(), 7);
}

#[test]
#[ignore]
fn reply_exception() {
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
#[ignore]
fn close() {
    let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.close();
}

#[test]
#[ignore]
fn free() {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    modbus.free();
}

// Timeout tests
#[test]
fn timeout_default() {
    let timeout: Timeout = Default::default();
    assert_eq!(timeout, Timeout { sec: 0, usec: 0 });
}

#[test]
fn timeout_new() {
    let timeout = Timeout::new(1, 2);
    assert_eq!(timeout, Timeout { sec: 1, usec: 2 });
}

#[test]
fn timeout_new_sec() {
    let timeout = Timeout::new_sec(1);
    assert_eq!(timeout, Timeout { sec: 1, usec: 0 });
}

#[test]
fn timeout_new_usec() {
    let timeout = Timeout::new_usec(2);
    assert_eq!(timeout, Timeout { sec: 0, usec: 2 });
}
