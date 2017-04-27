extern crate pkg_config;

use std::env;
use std::fs::{self, File};
use std::io::prelude::*;
use std::path::{Path, PathBuf};
use std::process::Command;


macro_rules! t {
    ($e:expr) => (match $e{
        Ok(e) => e,
        Err(e) => panic!("{} failed with {}", stringify!($e), e),
    })
}

fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    if pkg_config::find_library("libmodbus").is_ok() {
        return
    }

    if !Path::new("libmodbus/.git").exists() {
        let _ = Command::new("git").args(&["submodule", "update", "--init"])
                                   .status();
    }

    let target = env::var("TARGET").unwrap();
    let host = env::var("HOST").unwrap();
    let windows = target.contains("windows");
    let msvc = target.contains("msvc");

    // Clean up OUTPUT dir
    let _ = fs::remove_dir_all(env::var("OUT_DIR").unwrap());
    t!(fs::create_dir_all(env::var("OUT_DIR").unwrap()));


    if target.contains("windows") {
        return
    }

    if target.contains("apple") {
        return
    }
}
