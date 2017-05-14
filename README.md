# libmodbus-rs
## [libmodbus](http://libmodbus.org/) bindings for Rust [![Crates.io version](https://img.shields.io/crates/v/libmodbus-rs.svg)](https://crates.io/crates/libmodbus-rs) [![Build Status](https://travis-ci.org/zzeroo/libmodbus-rs.svg?branch=master)](https://travis-ci.org/zzeroo/libmodbus-rs) [![Build status](https://ci.appveyor.com/api/projects/status/dfjyswsgj6menctw?svg=true)](https://ci.appveyor.com/project/zzeroo/libmodbus-rs)

[Homepage |][homepage]&nbsp;
[Documentation |][docu]&nbsp;
[original libmodbus Documentation |][libmodbus-docu]&nbsp;
[Repo auf Github.com |][repo]


```toml
[dependencies]
libmodbus = "0.4"
```

**This crate is in early beta state. Please don't use in production and expect odd behavior.**

This crate based on the latest libmodbus git:master branch. I plan to support the different libmodbus version via cargo's `feature` feature.

## Building libmodbus-rs

The libmobus ffi bindings (libmodbus-sys) are build using [bindgen][bindgen]. [Bindgen need Clang 3.9 or greater on your system.][bindgen-reg]

### Debian-based Linuxes

```sh
# apt-get install llvm-3.9-dev libclang-3.9-dev clang-3.9
```

### Arch

```sh
# pacman -S clang
```

For mor information about the bindgen requirements please visit [https://servo.github.io/rust-bindgen/requirements.html][bindgen-reg]

## Examples

All original libmodbus examples are reproduced in Rust. You can find them in the
`examples` directory of this crate.

* `random-test-server.rs` is necessary to launch a server before running `random-test-client.rs`. By default, it receives and replies to Modbus query on the localhost and port 1502.

* `random-test-client.rs` sends many different queries to a large range of addresses and values to test the communication between the client and the server.

* `unit-test-server.rs` and unit-test-client run a full unit test suite. These programs are essential to test the Modbus protocol implementation and libmodbus behavior.

* `bandwidth-server-one.rs`, `bandwidth-server-many-up.rs` and `bandwidth-client.rs` return very useful information about the performance of transfert rate between the server and the client. `bandwidth-server-one.rs` can only handles one connection at once with a client whereas `bandwidth-server-many-up.rs` opens a connection for each new clients (with a limit).

To start, for example, the random test server/ client use the following commands

```sh
cargo run --example random-test-server
```

In another shell start the client after the server
```sh
cargo run --example random-test-client
```


```sh
$ git clone https://github.com/zzeroo/libmodbus-rs
$ cd libmodbus-rs
$ cargo build
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
[docu]: http://zzeroo.github.io/libmodbus-rs/libmodbus_rs/index.html
[libmodbus]: http://libmodbus.org
[libmodbus-repo]: https://github.com/stephane/libmodbus.git
[libmodbus-docu]: http://zzeroo.github.io/libmodbus-rs/libmodbus/libmodbus.html
[bindgen]: https://github.com/servo/rust-bindgen
[bindgen-reg]: https://servo.github.io/rust-bindgen/requirements.html
