#!/bin/bash
set -e

echo sudo socat PTY,link=/dev/ttyS10 PTY,link=/dev/ttyS11 &
echo sudo chown $USER: /dev/ttyS{10,11}
echo cargo run --example simple-server -- -b rtu -s /dev/ttyS11 &
echo cargo run --example simple-client -- -b rtu -s /dev/ttyS10 &
echo sudo killall socat
echo sudo killall cargo