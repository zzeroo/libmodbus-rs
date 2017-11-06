// FIXME: Not working ATM

extern crate clap;

use clap::{Arg, ArgMatches, App};
use std::process::Command;


#[derive(Debug)]
enum Lang {
    C,
    Rust,
}
#[derive(Debug)]
struct Server(Lang);
#[derive(Debug)]
struct Client(Lang);



fn run(matches: &ArgMatches) -> Result<(), String> {
    let server = match matches.value_of("server").unwrap() {
        "c" => Server(Lang::C),
        "rust" => Server(Lang::Rust),
        _ => unreachable!(),
    };
    let client = match matches.value_of("client").unwrap() {
        "c" => Client(Lang::C),
        "rust" => Client(Lang::Rust),
        _ => unreachable!(),
    };

    match server.0 {
        Lang::Rust => {
            Command::new("./target/debug/examples/unit-test-server")
                .args(&[">/dev/null", "&"])
                .output()
                .expect("failed to execute process");
        },
        Lang::C => {
            Command::new("./libmodbus-sys/libmodbus/tests/unit-test-server")
                .args(&[">/dev/null", "&"])
                .output()
                .expect("failed to execute process");
        },
    }
    match client.0 {
        Lang::Rust => {
            Command::new("./target/debug/examples/unit-test-client")
                .output()
                .expect("failed to execute process");
        },
        Lang::C => {
            Command::new("./libmodbus-sys/libmodbus/tests/unit-test-client")
                .output()
                .expect("failed to execute process");
        },
    }


    Ok(())
}

fn main() {
    let matches = App::new("Start unit-test client/ server")
        .version(env!("CARGO_PKG_VERSION"))
        .arg(Arg::with_name("server")
            .help("Which server should be used")
            .long("server")
            .short("s")
            .possible_values(&["c", "rust"])
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("client")
            .help("Which client should be used")
            .long("client")
            .short("c")
            .possible_values(&["c", "rust"])
            .takes_value(true)
            .required(true))
        .get_matches();


    if let Err(ref err) = run(&matches) {
        println!("Error: {}", err);

        std::process::exit(1)
    }
}
