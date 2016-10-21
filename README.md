# libmodbus-rs

[![Build Status](https://travis-ci.org/zzeroo/libmodbus-rs.svg?branch=master)](https://travis-ci.org/zzeroo/libmodbus-rs)
[![Build status](https://ci.appveyor.com/api/projects/status/2vhl3qpoetryhiyf?svg=true)](https://ci.appveyor.com/project/zzeroo/libmodbus-rs)

[Documentation][dox]
[dox]: http://zzeroo.com


# Development Environment
## libmodbus
For development environments install libmodbus system wide.

```bash
cd
git clone https://github.com/stephane/libmodbus.git
cd libmodbus
./autogen.sh
./configure --prefix=/usr --disable-tests
make -j3 && make install
```

## libmodbus-rs

```bash
cd
git clone https://github.com/zzeroo/libmodbus-rs.git
cd libmodbus-rs
cargo run --example master -- /dev/ttyUSB0 1
```


# Links
* https://doc.rust-lang.org/book/ffi.html
* http://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html
* http://siciarz.net/ffi-rust-writing-bindings-libcpuid/
