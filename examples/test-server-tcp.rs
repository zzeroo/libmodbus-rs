// This test is not part of the original libmodbus lib!
//
// It shows how to use the ModbusTCPPI context.
//
use libmodbus::{Modbus, ModbusMapping, ModbusServer, ModbusTCP};


fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502)?;
    let mut socket = modbus.tcp_listen(1)?;
    modbus.tcp_accept(&mut socket)?;

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500)?;
    let mut query = vec![0u8; Modbus::MAX_ADU_LENGTH as usize];

    loop {
        let request_len = modbus.receive(&mut query)?;
        modbus.reply(&query, request_len, &modbus_mapping)?;
    }
}

fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
