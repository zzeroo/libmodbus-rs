# TODOs

## Urgend

- Error Handling
[] remove failure crate
[] create custom error or use another crate 'fehler'
[] fix strange error in RTU
    ```bash
    sudo socat PTY,link=/dev/ttyS10 PTY,link=/dev/ttyS11 &
    # this works only one time. The server then process strange data
    cargo run --example simple-server -- -b rtu -s /dev/ttyS10 &
    cargo run --example simple-client -- -b rtu -s /dev/ttyS11 &

    # If you change the client/ server ports all is fine
    cargo run --example simple-server -- -b rtu -s /dev/ttyS11 &
    cargo run --example simple-client -- -b rtu -s /dev/ttyS10 &
    ```