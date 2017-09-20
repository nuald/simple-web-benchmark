# Simple Web Benchmark

A simple web benchmark of Go, Rust, Scala and Node.js.

## Performance Tests

The stats gathered by the [hey](https://github.com/rakyll/hey) tool:

    sh -c "$GOPATH/bin/hey -n 50000 -c 256 -t 10 'http://127.0.0.1:3000/'"
    sh -c "$GOPATH/bin/hey -n 50000 -c 256 -t 10 'http://127.0.0.1:3000/greeting/hello'"

### MacOS Note

By default, MacOS has low limits on the number of concurrent connections, so
few kernel parameters tweak may be required:

    sudo sysctl -w kern.ipc.somaxconn=12000
    sudo sysctl -w kern.maxfilesperproc=1048576
    sudo sysctl -w kern.maxfiles=1148576

## Servers run instructions

Please change the required directory before running the server.

### Go

    go run main.go

### Rust

    cargo run --release

### Scala

    sbt run

### Node.js

    node main.js
