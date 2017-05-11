# libmodbus-rs
## [libmodbus](http://libmodbus.org/) bindings for Rust [![Build Status](https://travis-ci.org/zzeroo/libmodbus-rs.svg?branch=master)](https://travis-ci.org/zzeroo/libmodbus-rs) [![Build status](https://ci.appveyor.com/api/projects/status/dfjyswsgj6menctw?svg=true)](https://ci.appveyor.com/project/zzeroo/libmodbus-rs)

[Projektseite |][homepage]&nbsp;[Dokumentation |][doku]&nbsp;[Repo auf Github.com |][repo]


```toml
[dependencies]
libmodbus = "0.4"
```

## Building libmodbus-rs

For building libmodbus-rs you need a build environment with autoconf, libtool and clang.

Under debian/ ubuntu you can use this command to install this dependencies:

```sh
apt-get update
apt-get upgrade -yq
# Build tools
apt-get install -yyq build-essential autoconf libtool libclang-dev
# Additional tools
apt-get install -yyq curl git
```

```sh
$ git clone https://github.com/zzeroo/libmodbus-rs
$ cd libmodbus-rs
$ cargo build
```

# License
`libmodbus-rs` is distributed under the terms of the LGPL-2.1 license,
which is the same license, [libmodbus](http://libmodbus.org/) is using.


# Links
* http://libmodbus.org
* https://github.com/stephane/libmodbus.git
* https://doc.rust-lang.org/book/ffi.html
* http://blog.rust-lang.org/2015/04/24/Rust-Once-Run-Everywhere.html
* http://siciarz.net/ffi-rust-writing-bindings-libcpuid

This project hosts the original libmodbus documentation, used here, as well. Please have a look at http://zzeroo.github.io/libmodbus-rs/libmodbus/libmodbus.html.

[homepage]: http://zzeroo.github.io/libmodbus-rs
[repo]: https://github.com/zzeroo/libmodbus-rs
[doku]: http://zzeroo.github.io/libmodbus-rs/libmodbus_rs/index.html
[doku-libmodbus]: http://zzeroo.github.io/libmodbus-rs/libmodbus/libmodbus.html
[libmodbus]: http://libmodbus.org
