extern crate bindgen;
extern crate make_cmd;
extern crate pkg_config;

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

const LIBMODBUS_DIR: &'static str = "libmodbus";

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();

    let build_dir = Path::new(LIBMODBUS_DIR);
    let prefix    = Path::new(&out_dir).join("libmodbus-root");
    let include   = Path::new(&prefix).join("include")
                                      .join("modbus");

    // if `pkg-config` is present and the libmodbus headers are found
    // we use `pkg-config` to find the include_path and call bindgen with it.
    //

    if let Ok(library) = pkg_config::find_library("libmodbus") {
        if let Some(include) = library.include_paths.get(0) {
            run_bindgen(&include);
        }

        return
    }

    // pkg-config is not found. We build libmodbus (source is in git submodule) from source
    // and run bindgen with that folder as include path.
    //

    // If autogen.sh is not present, initalize git submodules
    if !Path::new("libmodbus/autogen.sh").exists() {
        run_command("", Command::new("git").args(&["submodule", "update", "--init"]));
    }

    // Generate configure, run configure, make, make install
    run_command("Generating configure",
        Command::new("./autogen.sh")
            .current_dir(&build_dir));

    run_command("Configuring libmodbus",
        Command::new("./configure")
            .arg("--prefix")
            .arg(prefix)
            .arg("--without-documentation")
            .current_dir(&build_dir));


    run_command("Building libmodbus",
        Command::new("make")
            .arg("install")
            .current_dir(&build_dir));

    run_bindgen(&include);
}


fn run_bindgen(include: &PathBuf) {
    let include_path = format!("-I{}", include.display());

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // Do not generate unstable Rust code that
        // requires a nightly rustc and enabling
        // unstable features.
        .no_unstable_rust()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_arg(include_path)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");


    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

// Helper which run a given command, and check it's success
fn run_command(which: &'static str, cmd: &mut Command) {
    assert!(cmd.status().expect(which).success(), which);
}
