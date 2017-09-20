# simple-web-benchmark

A simple web benchmark of Go, Rust, Scala and Node.js.

## Performance tests

The stats gathered by the [hey](https://github.com/rakyll/hey) tool:

    > sh -c "$GOPATH/bin/hey -n 50000 -c 256 -t 10 'http://127.0.0.1:3000/greeting/hello'"

## Servers run instructions

Please change the required directory before running the server.

### Go

    > go run main.go

### Rust

    > cargo run --release

### Scala

    > sbt run

### Node.js

    > node main.js
