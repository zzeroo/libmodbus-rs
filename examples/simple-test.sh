#!/bin/bash

set -e

clean() {
  /usr/bin/pkill simple-server || :
  /usr/bin/pkill simple-client || :
}

cargo build --example simple-server
cargo build --example simple-client



echo -e "\nTest Rust binaries"
clean
./target/debug/examples/simple-server --backend tcp >/dev/null &
sleep 1
./target/debug/examples/simple-client --backend tcp

