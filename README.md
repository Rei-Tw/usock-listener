# usock-listener

Creates and listen to an Unix socket.

This is made to work with DDNet's logging feature (https://github.com/ddnet/ddnet).
This project supports only IPv4 logging. The rest is not handled.

When using DDNet server you have to set `sv_conn_logging_server` variable as the Unix socket file path.

The Unix socket should always receive data of size 23 bytes.

```
 0                   1                   2                   3
 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1 2 3 4 5 6 7 8 9 0 1
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|    Command    |               Type                            |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|               |               Address                         |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|               |               Unused (bcs v4)                 |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|                                                               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
|               |             Port              |               |
+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+-+
```

- Command : https://github.com/ddnet/ddnet/blob/master/src/engine/server/server.h#L480
    - `0x1` : Client joined the game 
    - `0x2` : Client left the game (Not handled)
- Type : https://github.com/ddnet/ddnet/blob/master/src/base/system.h#L821
    - `0x1` : IPv4
- Address : Client's IP address, sent after to the whitelist using POST request (Rest API)
- Port : Client's UDP port
 
## How to build
- Install rust binaries using rustup : `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- `cargo build --release`
Info : You may probably need to install `libssl-dev`.

## How to run it
To run it, execute `cargo run </path/to/unix.sock>`. The path is optional and will default to the same directory as `listener.sock`.
