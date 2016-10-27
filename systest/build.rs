extern crate ctest;

use std::env;
use std::path::PathBuf;

fn main() {
    let mut cfg = ctest::TestGenerator::new();

    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    cfg.header("modbus.h")
    .header("modbus-tcp.h")
    .header("modbus-private.h")
    .header("modbus-rtu.h");

    cfg.include(out_dir.join("include"));

    cfg.generate("../libmodbus-sys/lib.rs", "all.rs")
}
