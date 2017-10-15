extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP};


fn start_server() {
    use libmodbus_rs::{ModbusMapping, ModbusServer};

    let _server_handle = ::std::thread::spawn(move || {
        match Modbus::new_tcp("127.0.0.1", 1502) {
            Ok(mut server) => {
                let mapping = ModbusMapping::new(10, 10, 10, 10).expect("could not create modbus mapping");
                let mut socket = server.tcp_listen(1).expect("could not listen");
                server.tcp_accept(&mut socket).expect("unable to accept TCP socket");
                let mut query = vec![0; Modbus::MAX_ADU_LENGTH];
                match server.receive(&mut query) {
                    Ok(_request) => server.reply(&mut query, 1, &mapping),
                    Err(err) => Err(err),
                }
            },
            Err(err) => panic!("Could not create server: {}", err),
        }
    });
    // give server some time to come up
    ::std::thread::sleep(::std::time::Duration::from_millis(100));
}

#[test]
fn read_bits() {
    // Start modbus server
    start_server();

    // connect client
    match Modbus::new_tcp("127.0.0.1", 1502) {
        Ok(client) => {
            client.connect().expect("could not connect");
            assert_eq!(client.read_bits(0, 1).unwrap(), vec![0u8]);
        },
        _ => panic!("could not connect"),
    }
}

#[test]
#[ignore]
fn read_input_bits() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, num_bit: i32, dest: &mut [u8]) -> Result<i32>;
}

#[test]
#[ignore]
fn read_registers() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, num_bit: i32, dest: &mut [u16]) -> Result<i32>;
}

#[test]
#[ignore]
fn read_input_registers() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, num_bit: i32, dest: &mut [u16]) -> Result<i32>;
}

#[test]
#[ignore]
fn report_slave_id() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, max_dest: i32, dest: &mut [u8]) -> Result<i32>;
}

#[test]
#[ignore]
fn write_bit() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, status: bool) -> Result<i32>;
}

#[test]
#[ignore]
fn write_bits() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, num_bit: i32, src: &[u8]) -> Result<i32>;
}

#[test]
#[ignore]
fn write_register() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, value: i32) -> Result<i32>;
}

#[test]
#[ignore]
fn write_registers() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, address: i32, num_bit: i32, src: &[u16]) -> Result<i32>;
}

#[test]
#[ignore]
fn write_and_read_registers() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, write_address: i32, write_num_bit: i32, src: &[u16], read_address: i32,
}

#[test]
#[ignore]
fn send_raw_request() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, raw_request: &mut [u8]) -> Result<i32>;
}

#[test]
#[ignore]
fn receive_confirmation() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, response: &mut [u8]) -> Result<i32>;
}

#[test]
#[ignore]
fn reply_exception() {
    // let modbus = Modbus::new_tcp("127.0.0.1", 1502).unwrap();
    // let address = 1;
    // let mut dest = vec![0u8; 100];
    //
    // assert_eq!(modbus.read_bits(address, 1, &mut dest).unwrap(), 1);
    // &self, request: &[u8], exception_code: u32) -> Result<i32>;
}
