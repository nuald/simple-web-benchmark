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

### Automation

Please use the Scala script to run all the test automatically:

    scala run.scala

### Preliminary Results

Hardware: MacBook Pro (CPU: 2.3 GHz Intel Core i7, Mem: 16 GB 1600 MHz DDR3)

Software: Go 1.9, Rust 1.20.0, Scala 2.12.3, Node.js v8.5.0, D v2.076.0

Results for http://127.0.0.1:3000/:

| Language | Average, secs | Requests/sec |
|----------|---------------|--------------|
| Go       | 0.0041        | 61263        |
| Rust     | 0.0061        | 41174        |
| Scala    | 0.0063        | 28951        |
| Node.js  | 0.0074        | 31451        |
| D        | 0.0091        | 25009        |

Results for http://127.0.0.1:3000/greeting/hello:

| Language | Average, secs | Requests/sec |
|----------|---------------|--------------|
| Go       | 0.0046        | 53842        |
| Rust     | 0.0082        | 30847        |
| Scala    | 0.0063        | 35139        |
| Node.js  | 0.0066        | 31006        |
| D        | 0.0085        | 25166        |

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

