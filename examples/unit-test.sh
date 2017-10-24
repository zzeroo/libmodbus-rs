#!/bin/bash

set -e

clean() {
  /usr/bin/pkill unit-test-server || :
  /usr/bin/pkill unit-test-client || :
}

cargo build --example unit-test-server
cargo build --example unit-test-client



# echo -e "\nTest Rust binaries"
# clean
# ./target/debug/examples/unit-test-server >/dev/null &
# sleep 1
# ./target/debug/examples/unit-test-client


# echo -e "\nTest libmodbus C binaries"
# clean
# ./libmodbus-sys/libmodbus/tests/unit-test-server >/dev/null &
# sleep 1
# ./libmodbus-sys/libmodbus/tests/unit-test-client


echo -e "\nTest libmodbus C server and Rust client"
clean
./libmodbus-sys/libmodbus/tests/unit-test-server >/dev/null &
sleep 1
./target/debug/examples/unit-test-client


# echo -e "\nTest Rust server and libmodbus C client"
# clean
# ./target/debug/examples/unit-test-server >/dev/null &
# sleep 1
# ./libmodbus-sys/libmodbus/tests/unit-test-client
