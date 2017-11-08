extern crate libmodbus_rs;
extern crate time;

mod unit_test_config;

use libmodbus_rs::{Modbus, ModbusClient, ModbusTCP, ModbusRTU};
use std::env;
use std::io::Error;
use std::mem::size_of;
use std::process::exit;
use time::PreciseTime;
use unit_test_config::*;


const G_MSEC_PER_SEC: i64 = 1_000;

fn run() -> Result<(), std::io::Error> {
    let args: Vec<_> = env::args().collect();
    let (backend, n_loop) = if args.len() > 1 {
        match args[1].to_lowercase().as_ref() {
            "tcp" => (Backend::TCP, 100_000),
            "rtu" => (Backend::RTU, 100),
            _ => {
                println!("Usage:\n  {} [tcp|rtu] - Modbus client to measure data bandwith\n\n", args[0]);
                std::process::exit(-1);
            }
        }
    } else {
        (Backend::TCP, 100_000)
    };

    let mut modbus;
    if backend == Backend::TCP {
        modbus = Modbus::new_tcp("127.0.0.1", 1502).expect("Could not create TCP context");
    } else {
        modbus = Modbus::new_rtu("/dev/ttyUSB1", 115200, 'N', 8, 1).expect("Could not create RTU context");
        modbus.set_slave(1).expect("Could not set slave id");
    }

    match modbus.connect() {
        Err(_) => {
            println!("Connection failed: {}", Modbus::strerror(Error::last_os_error().raw_os_error().unwrap()));
            exit(-1)
        }
        _ => {}
    }

    /* Allocate and initialize the memory to store the status */
    let mut tab_bit = vec![0u8; Modbus::MAX_READ_BITS as usize * size_of::<u8>()];

    /* Allocate and initialize the memory to store the registers */
    let mut tab_reg = vec![0u16; Modbus::MAX_READ_REGISTERS as usize * size_of::<u16>()];

    println!("READ BITS\n");

    let nb_points = Modbus::MAX_READ_BITS as u16;
    let start = PreciseTime::now();
    for _ in 0..n_loop {
        let rc = modbus.read_bits(0, nb_points, &mut tab_bit);
        if rc.is_err() {
            return Err(Error::last_os_error());
        }
    }
    let end = PreciseTime::now();
    let elapsed = start.to(end).num_milliseconds();

    let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    println!("Transfert rate in points/seconds:");
    println!("* {} points/s", rate);
    println!();

    let bytes = n_loop * (nb_points as i64 / 8) + ( if nb_points % 8 == 0 { 1 } else { 0 } );
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("Values:");
    println!("* {} x {} values", n_loop, nb_points);
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!();

    /* TCP: Query and reponse header and values */
    let bytes = 12 + 9 + (nb_points as i64 / 8) + ( if nb_points % 8 == 0 { 1 } else { 0 } );
    println!("Values and TCP Modbus overhead:");
    println!("* {} x {} bytes", n_loop, bytes);
    let bytes = n_loop * bytes;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("\n");

    println!("READ REGISTERS\n");

    let nb_points = Modbus::MAX_READ_REGISTERS as u16;
    let start = PreciseTime::now();
    for _ in 0..n_loop {
        let rc = modbus.read_registers(0, nb_points, &mut tab_reg);
        if rc.is_err() {
            return Err(Error::last_os_error());
        }
    }
    let end = PreciseTime::now();
    let elapsed = start.to(end).num_milliseconds();

    let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    println!("Transfert rate in points/seconds:");
    println!("* {} registers/s", rate);
    println!();

    let bytes = n_loop * nb_points as i64 * size_of::<u16>() as i64;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("Values:");
    println!("* {} x {} values", n_loop, nb_points);
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("");

    /* TCP:Query and reponse header and values */
    let bytes = 12 + 9 + (nb_points as i64 * size_of::<u16>() as i64);
    println!("Values and TCP Modbus overhead:");
    println!("* {} x {} bytes", n_loop, bytes);
    let bytes = n_loop * bytes;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("\n");

    println!("WRITE AND READ REGISTERS\n");

    let nb_points = Modbus::MAX_WR_WRITE_REGISTERS as u16;
    let start = PreciseTime::now();
    for _ in 0..n_loop {
        let rc = modbus.write_and_read_registers(0, nb_points, &tab_reg.clone(), // FIXME: this clone costs to much
                                                 0, nb_points, &mut tab_reg);
        if rc.is_err() {
            return Err(Error::last_os_error());
        }
    }
    let end = PreciseTime::now();
    let elapsed = start.to(end).num_milliseconds();

    let rate = (n_loop * nb_points as i64) * G_MSEC_PER_SEC / elapsed;
    println!("Transfert rate in points/seconds:");
    println!("* {} registers/s", rate);
    println!("");

    let bytes = n_loop * nb_points as i64 * size_of::<u16>() as i64;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("Values:");
    println!("* {} x {} values", n_loop, nb_points);
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("");

    /* TCP:Query and reponse header and values */
    let bytes = 12 + 9 + (nb_points as i64 * size_of::<u16>() as i64);
    println!("Values and TCP Modbus overhead:");
    println!("* {} x {} bytes", n_loop, bytes);
    let bytes = n_loop * bytes;
    let rate = bytes / 1024 * G_MSEC_PER_SEC / elapsed;
    println!("* {:.3} ms for {} bytes", elapsed, bytes);
    println!("* {} KiB/s", rate);
    println!("");



    Ok(())
}



fn main() {
    if let Err(ref err) = run() {
        println!("{}", Modbus::strerror(err.raw_os_error().unwrap()));

        std::process::exit(1)
    }
}
