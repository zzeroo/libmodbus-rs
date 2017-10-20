#!/bin/bash

set -e

clean() {
  /usr/bin/pkill random-test-server || :
  /usr/bin/pkill random-test-client || :
}

cargo build --example random-test-server
cargo build --example random-test-client



echo -e "\nTest Rust binaries"
clean
./target/debug/examples/random-test-server >/dev/null &
sleep 1
./target/debug/examples/random-test-client | grep "Test: SUCCESS" || exit 1


echo -e "\nTest libmodbus C binaries"
clean
./libmodbus-sys/libmodbus/tests/random-test-server >/dev/null &
sleep 1
./libmodbus-sys/libmodbus/tests/random-test-client | grep "Test: SUCCESS" || exit 1


echo -e "\nTest libmodbus C server and Rust client"
clean
./libmodbus-sys/libmodbus/tests/random-test-server >/dev/null &
sleep 1
./target/debug/examples/random-test-client | grep "Test: SUCCESS" || exit 1


echo -e "\nTest Rust server and libmodbus C client"
clean
./target/debug/examples/random-test-server >/dev/null &
sleep 1
./libmodbus-sys/libmodbus/tests/random-test-client | grep "Test: SUCCESS" || exit 1
