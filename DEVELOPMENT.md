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

## modbus-rs

```bash
cd
git clone https://github.com/zzeroo/modbus-rs.git
cd modbus-rs
cargo run --example master -- /dev/ttyUSB0 1
```
