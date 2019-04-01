/// This build.rs build script checks if the libmodbus git submodule is checked out. Then libmodbus
/// is build from source. Subsequently bindgen is called on top of the self builded libmodbus.
extern crate bindgen;
extern crate cc;

use bindgen::builder;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;


// stolen from: https://github.com/alexcrichton/backtrace-rs/blob/master/backtrace-sys/build.rs
macro_rules! t {
    ($e:expr) => (match $e{
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}


const LIBMODBUS_DIR: &'static str = "libmodbus";
fn main() {
    let dst = env::var("OUT_DIR").unwrap();
    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();

    let build_dir = Path::new(LIBMODBUS_DIR);
    let prefix = Path::new(&dst).join("libmodbus-root");
    let include = Path::new(&prefix)
        .join("include")
        .join("modbus");

    // If autogen.sh is not present, initalize git submodules
    if !Path::new("libmodbus/autogen.sh").exists() {
        run_command("", Command::new("git").args(&["submodule", "update", "--init"]));
    }

    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));

    let cfg = cc::Build::new();
    let compiler = cfg.get_compiler();
    let mut flags = OsString::new();
    for (i, flag) in compiler.args().iter().enumerate() {
        if i > 0 {
            flags.push(" ");
        }
        flags.push(flag);
    }

    // Generate configure, run configure, make, make install
    run_command("Generating configure",
                Command::new("autoreconf")
                    .arg("--install")
                    .arg("--symlink")
                    .arg("--force")
                    .current_dir(&build_dir));

    run_command("Configuring libmodbus",
                Command::new("./configure")
                    .arg("--prefix")
                    .arg(&prefix)
                    .env("CC", compiler.path())
                    .env("CFLAGS", flags)
                    .arg("--with-pic")
                    .arg("--disable-shared")
                    .arg("--disable-tests")
                    .arg(format!("--target={}", target))
                    .arg(format!("--host={}", host))
                    .current_dir(&build_dir));

    run_command("Building libmodbus",
                Command::new("make")
                    .arg("install")
                    .current_dir(&build_dir));

    // Static link
    println!("cargo:rustc-link-lib=static=modbus");
    println!("cargo:rustc-link-search=native={}/libmodbus-root/lib", dst);

    // Build Rust bindings
    run_bindgen(&include);
}

fn run_bindgen(include: &PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("can't access $OUT_DIR"));
    // Configure and generate bindings.
    let bindings = builder()
        .header("wrapper.h")
        .clang_arg(format!("-I{}", include.display()))
        .bitfield_enum("modbus_error_recovery_mode")
        .generate()
        .expect("could not reate binding");

    // Write the generated bindings to an output file.
    bindings.write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// Helper which run a given command, and check it's success
fn run_command(which: &'static str, cmd: &mut Command) {
    assert!(cmd.status().expect(which).success(), which);
}
