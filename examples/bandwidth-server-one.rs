mod unit_test_config;

use libmodbus::{Modbus, ModbusMapping, ModbusServer, ModbusTCP, ModbusRTU};
use std::env;
use unit_test_config::*;


fn run() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    let backend = if args.len() > 1 {
        match args[1].to_lowercase().as_ref() {
            "tcp" => Backend::TCP,
            "rtu" => Backend::RTU,
            _ => {
                println!("Usage:\n  {} [tcp|rtu] - Modbus client to measure data bandwith\n", args[0]);
                std::process::exit(-1);
            }
        }
    } else {
        Backend::TCP
    };

    let mut modbus;
    let mut socket;
    if backend == Backend::TCP {
        modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("Could not create TCP context");
        socket = modbus.tcp_listen(1).expect("Could not listen to TCP socket");
        modbus.tcp_accept(&mut socket).unwrap();
    } else {
        modbus = Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1).expect("Could not create RTU context");
        modbus.set_slave(1).unwrap();
        modbus.connect().unwrap();
    }

    let mb_mapping = ModbusMapping::new(Modbus::MAX_READ_BITS, 0,
                                        Modbus::MAX_READ_REGISTERS, 0).expect("Failed to allocate the mapping");

    loop {
        let mut query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH];

        match modbus.receive(&mut query) {
            Ok(rc) => modbus.reply(&query, rc, &mb_mapping),
            Err(err) => {
                println!("Quit the loop: {}", err);
                break;
            }
        }.unwrap();
    }

    Ok(())
}

fn main() {
    if let Err(ref err) = run() {
        println!("{}", Modbus::strerror(err.raw_os_error().unwrap()));

        std::process::exit(1)
    }
}
