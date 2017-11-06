#!/bin/bash

set -e

# Kill unit-test server and client with allways true result (`|| :` means `or :`, where `:` returns everytime true)
clean() {
  /usr/bin/pkill unit-test-server || :
  /usr/bin/pkill unit-test-client || :
}
# Diff output if both .output files are present
diff_output() {
  if [ -f unti-test-client-C.output ] && [ -f unti-test-client-rust.output ]; then
    diff --color=always unti-test-client-C.output unti-test-client-rust.output
  fi
}
build_rust() {
  cargo build --example unit-test-server
  cargo build --example unit-test-client
}

run_rust() {
  echo -e "\nTest Rust binaries"
  clean
  build_rust
  ./target/debug/examples/unit-test-server >/dev/null 2>&1 &
  sleep 1
  ./target/debug/examples/unit-test-client
}

run_c() {
  echo -e "\nTest libmodbus C binaries"
  clean
  ./libmodbus-sys/libmodbus/tests/unit-test-server >/dev/null 2>&1 &
  sleep 1
  ./libmodbus-sys/libmodbus/tests/unit-test-client
}

run_c_server_rust_client() {
  echo -e "\nTest libmodbus C server and Rust client"
  clean
  build_rust
  ./libmodbus-sys/libmodbus/tests/unit-test-server >/dev/null 2>&1 &
  sleep 1
  ./target/debug/examples/unit-test-client
}

run_rust_server_c_client() {
  echo -e "\nTest Rust server and libmodbus C client"
  clean
  build_rust
  ./target/debug/examples/unit-test-server >/dev/null 2>&1 &
  sleep 1
  ./libmodbus-sys/libmodbus/tests/unit-test-client
}

cleanup() {
  [ -f unti-test-client-C.output ] && rm unti-test-client-C.output
  [ -f unti-test-client-rust.output ] && rm unti-test-client-rust.output
}

usage() {
  echo -e "Helper script to run the unit-test-. client and server\n"
  echo -e "Without a parameter the C server and the Rust client are started."
  echo -e "All other combinations can be done via the following parameters.\n"
  echo -e "Usage:"
  echo -e "\t `basename $0` [--rust][--c][--rust_server_c_client][--c_server_rust_client]\n"
  echo -e "\t--rust\t\t\tRust server and client"
  echo -e "\t--c\t\t\tC server and client"
  echo -e "\t--rust_server_c_client\tRust server, C client"
  echo -e "\t--c_server_rust_client\tC server, Rust client (default)"
  echo

  exit 0
}


# main

# TODO: Fix diff view of results
# TODO: add run time stats
while test $# -gt 0
do
    case "$1" in
        --rust) run_rust
        exit 0
        ;;
        --c) run_c
        exit 0
        ;;
        --rust_server_c_client) run_rust_server_c_client
        exit 0
        ;;
        --c_server_rust_client) run_c_server_rust_client
        exit 0
        ;;
        --*) usage
        ;;
        *) usage
        ;;
    esac
    shift
done

# Default task
run_c_server_rust_client

exit 0
