// Parts stolen here: https://github.com/zonyitoo/libsodium-sys/blob/master/build.rs
// here: https://github.com/alexcrichton/git2-rs/blob/master/libgit2-sys/build.rs
// Manual here: http://doc.crates.io/build-script.html
extern crate gcc;
extern crate pkg_config;

use std::env;
use std::fs;
use std::path::{Path};
use std::process::{Command, Stdio};

#[allow(unused_variables)]
fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    //if env::var("LIBMODBUS_SYS_USE_PKG_CONFIG").is_ok() {
        if pkg_config::find_library("libmodbus").is_ok() {
            return
        }
    //}

    if !Path::new("libmodbus/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                    .status();
    }

    let cargo_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let output_dir = env::var("OUT_DIR").unwrap();

    let src = Path::new(&cargo_dir[..]);
    let dst = Path::new(&output_dir[..]);
    let target = env::var("TARGET").unwrap();

    let root = src.join("libmodbus");

    run(Command::new("sh")
            .arg("-c")
            .arg(&root.join("autogen.sh"))
            .current_dir(&root));

    let _ = fs::remove_dir_all(&dst.join("include"));
    let _ = fs::remove_dir_all(&dst.join("lib"));
    let _ = fs::remove_dir_all(&dst.join("build"));
    fs::create_dir(&dst.join("build")).unwrap();

    let mut config_opts = Vec::new();
    config_opts.push(format!("{:?}", root.join("configure")));
    config_opts.push(format!("--prefix={:?}", dst));

    run(Command::new("sh")
            .arg("-c")
            .arg(&config_opts.join(" "))
            .current_dir(&dst.join("build")));

    run(Command::new(make())
            .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
            .current_dir(&dst.join("build")));

    run(Command::new(make())
            .arg(&format!("-j{}", env::var("NUM_JOBS").unwrap()))
            .arg("install")
            .current_dir(&dst.join("build")));

    println!("cargo:rustc-flags=-L {}/lib -l modbus", dst.display());
    println!("cargo:root={}", dst.display());
    println!("cargo:include={}/include", dst.display());
}

fn make() -> &'static str {
    if cfg!(target_os = "freebsd") {"gmake"} else {"make"}
}

fn run(cmd: &mut Command) {
    println!("running: {:?}", cmd);
    assert!(cmd.stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status()
                .unwrap()
                .success());
}
