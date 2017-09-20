# Simple Web Benchmark

A simple web benchmark of Go, Rust, D, Scala and Node.js.

## Testing

The stats gathered by the [hey](https://github.com/rakyll/hey) tool (please run it twice for
the JIT optimizations where it's applicable):

    sh -c "$GOPATH/bin/hey -n 50000 -c 256 -t 10 'http://127.0.0.1:3000/'"
    sh -c "$GOPATH/bin/hey -n 50000 -c 256 -t 10 'http://127.0.0.1:3000/greeting/hello'"

### MacOS Note

By default, MacOS has low limits on the number of concurrent connections, so
few kernel parameters tweaks may be required:

    sudo sysctl -w kern.ipc.somaxconn=12000
    sudo sysctl -w kern.maxfilesperproc=1048576
    sudo sysctl -w kern.maxfiles=1148576

### Preliminary Results

Hardware: MacBook Pro (CPU: 2.3 GHz Intel Core i7, Mem: 16 GB 1600 MHz DDR3)

Software: Go 1.9, Rust 1.20.0, Scala 2.12.3, Node.js v8.5.0

Results for http://127.0.0.1:3000/:

| Language | Average, secs | Requests/sec |
|----------|---------------|--------------|
| Go       | 0.0041        | 61587        |
| Rust     | 0.0054        | 46337        |
| Scala    | 0.0066        | 34157        |
| Node.js  | 0.0070        | 34202        |

Results for http://127.0.0.1:3000/greeting/hello:

| Language | Average, secs | Requests/sec |
|----------|---------------|--------------|
| Go       | 0.0044        | 57509        |
| Rust     | 0.0059        | 42767        |
| Scala    | 0.0055        | 36823        |
| Node.js  | 0.0064        | 36792        |

## Usage

Please change the required directory before running the server.

### Go

    go run main.go

### Rust

    cargo run --release

### D

    dub run --build=release

### Scala

    sbt run

### Node.js

    node main.js

