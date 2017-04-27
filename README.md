# modbus-rs

[![Build Status](https://travis-ci.org/zzeroo/modbus-rs.svg?branch=master)](https://travis-ci.org/zzeroo/modbus-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/2vhl3qpoetryhiyf?svg=true)](https://ci.appveyor.com/project/zzeroo/modbus-rs)

[Documentation][dox]
[dox]: http://zzeroo.com

libmodbus bindings for Rust

```toml
[dependencies]
modbus = "0.6"
```

## Building modbus-rs

First, you'll need to install _clang_. Afterwards, just run:

```sh
$ git clone https://github.com/zzeroo/modbus-rs
$ cd modbus-rs
$ cargo build
```

# License
`modbus-rs` is distributed under the terms of the LGPL-2.1 license,
which is the same license, [libmodbus](http://libmodbus.org/) is using.



# Links
* http://libmodbus.org/
* https://github.com/stephane/libmodbus.git
* https://doc.rust-lang.org/book/ffi.html
* http://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html
* http://siciarz.net/ffi-rust-writing-bindings-libcpuid/
