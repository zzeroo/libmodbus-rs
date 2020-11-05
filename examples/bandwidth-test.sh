#!/bin/bash

set -e

clean() {
  /usr/bin/pkill bandwidth-server-one || :
  /usr/bin/pkill bandwidth-server-many-up || :
  /usr/bin/pkill bandwidth-client || :
}

cargo build --example bandwidth-server-one
cargo build --example bandwidth-server-many-up
cargo build --example bandwidth-client


#  One Server
echo -e "\nTest Rust binaries (One Server)"
clean
./target/debug/examples/bandwidth-server-one >/dev/null &
sleep 1
./target/debug/examples/bandwidth-client

echo -e "\nTest libmodbus C binaries (One Server)"
clean
./libmodbus-sys/libmodbus/tests/bandwidth-server-one >/dev/null &
sleep 1
./libmodbus-sys/libmodbus/tests/bandwidth-client

echo -e "\nTest libmodbus C server and Rust client (One Server)"
clean
./libmodbus-sys/libmodbus/tests/bandwidth-server-one >/dev/null &
sleep 1
./target/debug/examples/bandwidth-client

echo -e "\nTest Rust server and libmodbus C client (One Server)"
clean
./target/debug/examples/bandwidth-server-one >/dev/null &
sleep 1
./libmodbus-sys/libmodbus/tests/bandwidth-client