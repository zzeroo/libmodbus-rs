/// This build.rs build script checks if libmodbus is present if so, it calls bindgen against
/// the installed libmodbus.
/// If there is no libmodbus present, the libmodbus git submodule is checked out. Then libmodbus
/// is build from source. Subsequently bindgen is called then on top of libmodbus.
/// Dependencie are `clang` and `pkg-config`.
extern crate bindgen;
extern crate cc;
extern crate pkg_config;

use std::env;
use std::ffi::OsString;
use std::path::{Path, PathBuf};
use std::process::Command;

// stolen from: https://github.com/alexcrichton/backtrace-rs/blob/master/backtrace-sys/build.rs
#[allow(unused_macros)]
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

    // If `pkg-config` found libmodbus we use `pkg-config` to get
    // the include_path and call bindgen with that.
    if let Ok(library) = pkg_config::probe_library("libmodbus") {
        if let Some(include) = library.include_paths.get(0) {
            run_bindgen(&include);
        }

        return;
    }

    // `pkg-config` doesn't found libmodbus.
    // So we have to compile libmodbus from source (source are in a git submodule)
    // Then run bindgen with that folder as include path set.

    // If autogen.sh is not present, initalize git submodules
    if !Path::new("libmodbus/autogen.sh").exists() {
        run_command("", Command::new("git").args(&["submodule", "update", "--init"]));
    }

    // FIXME: Undocumented rmdir, I think this is not needed
    // t!(fs::remove_dir_all(env::var("OUT_DIR").unwrap()));
    // t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));

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
                Command::new("bash")
                    .arg("autogen.sh")
                    .current_dir(&build_dir));

    run_command("Configuring libmodbus",
                Command::new("./configure")
                    .arg("--prefix")
                    .arg(&prefix)
                    .env("CC", compiler.path())
                    .env("CFLAGS", flags)
                    .arg("--disable-shared")
                    .arg("--disable-tests")
                    .arg(format!("--target={}", target))
                    .arg(format!("--host={}", host))
                    .current_dir(&build_dir));

    run_command("Building libmodbus",
                Command::new("make")
                    .arg("install")
                    .current_dir(&build_dir));

    println!("cargo:rustc-link-lib=static=modbus");
    println!("cargo:rustc-link-search=native={}/libmodbus-root/lib", dst);

    // Tell cargo to invalidate the build crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    run_bindgen(&include);
}

fn run_bindgen(include: &PathBuf) {
    let out_path = PathBuf::from(env::var("OUT_DIR").expect("can't access $OUT_DIR"));
    // Configure and generate bindings.
    let bindings = bindgen::Builder::default()
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
