use clap::{App, Arg, ArgMatches};
use libmodbus::{Modbus, ModbusMapping, ModbusRTU, ModbusServer, ModbusTCP, ModbusTCPPI};

#[derive(Debug, Eq, PartialEq)]
enum Backend {
    TCP,
    TCPPI,
    RTU,
}

const SERVER_ID: u8 = 247;

fn run(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let backend;
    let mut modbus;
    let mut query;

    match matches.value_of("backend").unwrap() {
        "tcp" => backend = Backend::TCP,
        "tcppi" => backend = Backend::TCPPI,
        "rtu" => backend = Backend::RTU,
        _ => unreachable!(), // because clap ensures that for us
    }

    match backend {
        Backend::RTU => {
            query = vec![0u8; Modbus::RTU_MAX_ADU_LENGTH as usize];
            let serial_interface = matches
                .value_of("serial_interface")
                .unwrap_or("/dev/ttyUSB0");
            modbus = Modbus::new_rtu(&serial_interface, 9600, 'N', 8, 1)?;
            modbus.set_slave(SERVER_ID)?;
        }
        Backend::TCP => {
            query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
            modbus = Modbus::new_tcp("127.0.0.1", 1502)?;

            let mut socket = modbus.tcp_listen(1)?;
            modbus.tcp_accept(&mut socket)?;
        }
        Backend::TCPPI => {
            query = vec![0u8; Modbus::TCP_MAX_ADU_LENGTH as usize];
            modbus = Modbus::new_tcp_pi("::0", "1502")?;

            let mut socket = modbus.tcp_listen(1)?;
            modbus.tcp_accept(&mut socket)?;
        }
    }

    // modbus.set_debug(true)?;
    modbus.connect()?;

    let modbus_mapping = ModbusMapping::new(500, 500, 500, 500).unwrap();

    loop {
        match modbus.receive(&mut query) {
            Ok(num) => modbus.reply(&query, num, &modbus_mapping),
            Err(err) => {
                println!("ERROR while parsing: {}", err);
                break;
            }
        }
        .expect("could not receive message");
    }
    println!("Quit the loop: ");

    Ok(())
}

fn main() {
    let matches = App::new("simple-client")
        .version(env!("CARGO_PKG_VERSION"))
        .about("Simple Modbus Client with support for the different contextes (rtu, tcp, tcppi)!")
        .author("Stefan Müller (zzeroo) <s.mueller@it.kls-glt.de>")
        .arg(
            Arg::with_name("backend")
                .help("which backend shoud be used")
                .long("backend")
                .short("b")
                .possible_values(&["rtu", "tcp", "tcppi"])
                .takes_value(true)
                .required(true),
        )
        .arg(
            Arg::with_name("serial_interface")
                .help("which backend shoud be used")
                .long("serial_interface")
                .short("s")
                .takes_value(true)
                .required(false),
        )
        .get_matches();

    if let Err(ref err) = run(&matches) {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
