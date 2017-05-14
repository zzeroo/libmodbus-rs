# License

These examples are provided under BSD license (see associated LICENSE file).

# Usage

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

## simulate serial interface

If you want to try the ModbusRTU context you can use a pty ("pseudo-teletype", where a serial port is a "real teletype") for this. From one end, open /dev/ptyp5, and then attach your program to /dev/ttyp5; ttyp5 will act just like a serial port, but will send/receive everything it does via /dev/ptyp5.

If you really need it to talk to a file called /dev/ttys2, then simply move your old /dev/ttys2 out of the way and make a symlink from ptyp5 to ttys2.

Of course you can use some number other than ptyp5. Perhaps pick one with a high number to avoid duplicates, since all your login terminals will also be using ptys.

Wikipedia has more about ptys: http://en.wikipedia.org/wiki/Pseudo_terminal

This tip is from Stackoverflow user _apenwarr_ http://stackoverflow.com/questions/52187/virtual-serial-port-for-linux)
