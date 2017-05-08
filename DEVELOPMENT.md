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
