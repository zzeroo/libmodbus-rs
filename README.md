# libmodbus

> This crate was renamed to just *libmodbus*. Before version 1.0.0 the name of the crate was libmodbus-rs.

## [libmodbus](http://libmodbus.org/) bindings for Rust

Libmodbus is a free software library to send and receive data with a device that respects the Modbus protocol. That crate contains the Rust bindings for the libmodbus library (written in C).
Like libmodbus self, this crate supports serial port and Ethernet connections for Modbus server and clients.


[![Crates.io version](https://img.shields.io/crates/v/libmodbus-rs.svg)](https://crates.io/crates/libmodbus-rs) &nbsp;
[![Build Status](https://travis-ci.org/zzeroo/libmodbus-rs.svg?branch=master)](https://travis-ci.org/zzeroo/libmodbus-rs) &nbsp;
[![Coverage Status](https://coveralls.io/repos/github/zzeroo/libmodbus-rs/badge.svg?branch=master)](https://coveralls.io/github/zzeroo/libmodbus-rs?branch=master)

[Homepage |][homepage] &nbsp;
[Documentation |][doc] &nbsp;
[Original libmodbus documentation |][libmodbus-doc] &nbsp;
[Github repo |][repo]


This crate based on the latest libmodbus git:master branch. I plan to support the different libmodbus version via cargo's `feature` feature.

## Usage

Include the dependencies into your `Cargo.toml` file.

### for stable (crates.io)

```toml
[dependencies]
libmodbus-rs = "1"
```

### or development (github master)

```toml
[dependencies]
libmodbus-rs = { git = "https://github.com/zzeroo/libmodbus-rs.git" }
```

Some header files of the original libmodbus C library are recreated as traits (e.g. ModbusTCP, ModbusRTU, ModbusServer, ModbusClient, ...).
For example if you what to build an modbus server, in the modbus tcp context, include the following:

```rust
extern crate libmodbus_rs;

use libmodbus::{Modbus, ModbusServer, ModbusTCP};
```

The examples in the examples directory show this.

## Documentation

[Documentation (crates.io)][doc]

[Documentation (master)][doc-master]

## Building libmodbus-rs

The libmodbus ffi bindings (libmodbus-sys) are build using [bindgen][bindgen]. [Bindgen need Clang 3.9 or greater on your system.][bindgen-requirements]

### Dependencies Archlinux

```sh
pacman -S autoconf clang39 git libtool make
```

### Dependencies Debian based (e.g. Ubuntu)

```sh
apt install autoconf build-essential curl clang git-core libtool
```

Look also at the local ci/ docker files under `./ci/docker-archlinux` and `.ci/docker-debian9` for a known working, minimal setup.

For more information about the bindgen requirements please visit [https://servo.github.io/rust-bindgen/requirements.html][bindgen-requirements]

## Dependencies Windows

Follow the msys2 instructions <https://www.msys2.org>

```sh
pacman -Syu
```

Followed by the second update step

```sh
pacman -Su
```

Finally install clang

```sh
pacman -S  mingw64/mingw-w64-x86_64-clang
```

----

If all dependencies are solved, compile with `cargo build` and/ or run the tests with `cargo test`.

```sh
git clone https://github.com/zzeroo/libmodbus-rs
cd libmodbus-rs
cargo build
```

## Examples

Most of the original libmodbus examples are reproduced in Rust.
You can found them in the `examples` directory of this crate.

**Please have look at the README.md in the examples directory for more information about the examples.**

To start, for example, the random test server/ client use the following commands

```sh
cargo run --example random-test-server
```

In another shell start the client after the server

```sh
cargo run --example random-test-client
```

# License

`libmodbus-rs` is distributed under the terms of the LGPL-2.1 license, which is the same license, [libmodbus](http://libmodbus.org/) is using.

# Links

* [http://libmodbus.org][libmodbus]
* [https://github.com/stephane/libmodbus.git][libmodbus-repo]
* [https://github.com/servo/rust-bindgen][bindgen]
* [https://doc.rust-lang.org/book/ffi.html](https://doc.rust-lang.org/book/ffi.html)

This project hosts the original libmodbus documentation, used here, as well. Please have a look at http://zzeroo.github.io/libmodbus-rs/libmodbus/libmodbus.html.

[homepage]: http://zzeroo.github.io/libmodbus-rs
[repo]: https://github.com/zzeroo/libmodbus-rs
[doc]: https://docs.rs/crate/libmodbus-rs
[doc-master]: http://zzeroo.github.io/libmodbus-rs/libmodbus_rs/index.html
[libmodbus]: http://libmodbus.org
[libmodbus-repo]: https://github.com/stephane/libmodbus.git
[libmodbus-doc]: http://zzeroo.github.io/libmodbus-rs/libmodbus/libmodbus.html
[bindgen]: https://github.com/servo/rust-bindgen
[bindgen-requirements]: https://servo.github.io/rust-bindgen/requirements.html
