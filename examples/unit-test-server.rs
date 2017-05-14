extern crate libmodbus_rs;

use libmodbus_rs::{Modbus, ModbusServer};
use libmodbus_rs::errors::*; // for the `Result<T>` type
use std::env;


enum Backend {
    TCP,
    TCPPI,
    RTU,
}

fn run() -> Result<()> {
    let backend;
    let modbus: Modbus;

    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
        match args[1].to_lowercase().as_ref() {
            "tcp" => backend = Backend::TCP,
            "tcppi" => backend = Backend::TCPPI,
            "rtu" => backend = Backend::RTU,
            _ => {
                println!("Usage:\n  {} [tcp|tcppi|rtu] - Modbus server for unit testing\n\n", args[0]);
                std::process::exit(-1);
            }
        }
    } else {
        // By default
        backend = Backend::TCP;
    }

    let mut modbus = match backend {
        Backend::TCP => {
            use libmodbus_rs::ModbusTCP;

            Modbus::new_tcp("127.0.0.1", 1502)
        }
        Backend::TCPPI => {
            use libmodbus_rs::ModbusTCPPI;

            Modbus::new_tcp_pi("::0", "1502")
        }
        Backend::RTU => {
            use libmodbus_rs::ModbusRTU;

            Modbus::new_rtu("/dev/ttyUSB0", 115200, 'N', 8, 1)
        }
    }?;

    let header_lenght = modbus.get_header_length();
    println!("{:?}", header_lenght);

    let socket = modbus.get_socket();
    println!("{:?}", socket);

    Ok(())
}


fn main() {
    if let Err(ref err) = run() {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
