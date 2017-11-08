extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer, ModbusMapping, ModbusTCP};


fn run() -> Result<(), String> {
    let mut modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("could not create modbus TCP context");
    // modbus.set_debug(true).expect("could not set DEBUG mode");

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).expect("could not create Modbus Mapping");

    let mut socket = modbus.tcp_listen(1).expect("could not listen");
    modbus.tcp_accept(&mut socket).expect("could not create socket");


    loop {
        let mut query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];

        match modbus.receive(&mut query) {
                Ok(num) => modbus.reply(&query, num, &modbus_mapping),
                Err(err) => {
                    println!("ERROR while parsing: {}", err);
                    break;
                },
            }
            .expect("could not receive message");
    }
    println!("Quit the loop: ");

    // ModbusMapping and Modbus struct are dropped here, automatically by Rusts Drop trait.
    // Hence no manual `modbus_mapping_free(mb_mapping)`, `modbus_close(ctx)` nor `modbus_free(ctx)` needed, like in C.

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
