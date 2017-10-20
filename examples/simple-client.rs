// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;
extern crate clap;
extern crate libmodbus_rs;

mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain!{}
}

use errors::*;
use libmodbus_rs::{Modbus, ModbusRTU, ModbusTCP, ModbusTCPPI};
use clap::{App, Arg, ArgMatches};



#[derive(Debug, Eq, PartialEq)]
enum Backend {
    TCP,
    TCPPI,
    RTU,
}

const SERVER_ID: u8 = 247;

fn run(matches: &ArgMatches) -> Result<()> {
    let backend;
    let mut modbus: Modbus;

    match matches.value_of("backend").unwrap() {
        "tcp" => backend = Backend::TCP,
        "tcppi" => backend = Backend::TCPPI,
        "rtu" => backend = Backend::RTU,
        _ => unreachable!(), // because clap ensures that for us
    }

    match backend {
        Backend::RTU => {
            let serial_interface = matches.value_of("serial_interface").unwrap_or("/dev/ttyUSB1");
            modbus = Modbus::new_rtu(&serial_interface, 9600, 'N', 8, 1).chain_err(|| "unable to create modbus RTU context")?;
            modbus.set_slave(SERVER_ID).chain_err(|| format!("could not set slave address {}", SERVER_ID))?;
        },
        Backend::TCP => {
            modbus = Modbus::new_tcp("127.0.0.1", 1502).chain_err(|| "unable to create modbus TCP context")?;
        },
        Backend::TCPPI => {
            modbus = Modbus::new_tcp_pi("::1", "1502").chain_err(|| "unable to create modbus TCPv6 context")?;
        },
    }

    modbus.set_debug(true).chain_err(|| "could not set modbus DEBUG mode")?;
    modbus.connect().chain_err(|| "could not connect")?;

    // Work HERE

    Ok(())
}



fn main() {
    let matches = App::new("simple-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simple Modbus Client with support for the different contextes (rtu, tcp, tcppi)!")
        .author("Stefan Müller (zzeroo) <s.mueller@it.kls-glt.de>")
        .arg(Arg::with_name("backend")
            .help("which backend shoud be used")
            .long("backend")
            .short("b")
            .possible_values(&["rtu", "tcp", "tcppi"])
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("serial_interface")
            .help("which backend shoud be used")
            .long("serial_interface")
            .short("s")
            .takes_value(true)
            .required(false))
        .get_matches();

    if let Err(ref err) = run(&matches) {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
