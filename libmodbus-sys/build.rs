extern crate pkg_config;

use std::process::Command;


fn main() {
    let has_pkgconfig = Command::new("pkg-config").output().is_ok();

    println!("Rock n Roll");
}
