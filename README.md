# Simple Web Benchmark

A simple web benchmark of Crystal, D, Go, Java, Node.js, PHP, Python, Rust and Scala.

## Results

![SVG Plot](./suite/results/result.svg)

## Testing

The stats gathered by the [hey](https://github.com/rakyll/hey) tool (please run it twice for
the JIT optimizations where it's applicable):

    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/"
    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/greeting/hello"

### Using Docker

Build the image:

    $ docker build suite/ -t simple-web-benchmark

Enter the shell in the image:

    $ docker run -it --rm -v $(pwd):/src --network="host" simple-web-benchmark

There is a `Makefile` that could be used to simlify Docker usage:

 - `make build` (build the image);
 - `make shell` (enter the shell in the image).

Please note that the `make shell` rule requires `cpupower` utility installed
that is invoked with `sudo` to set cpufreq's performance governon
(it runs the CPU at the maximum frequence to eliminate throttling issues).

### Automation

Please use the Rust program to run the tests automatically:

    USAGE:
        cargo run --manifest-path suite/Cargo.toml -- [FLAGS] [OPTIONS] <lang>...

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information
            --verbose    Enables the verbose output

    OPTIONS:
        -o, --out <file>    Sets an image file to generate (result.svg by default, PNG/SVG/TSV are supported)

    ARGS:
        <lang>...    Sets the languages to test ('all' for all)

    The following languages are supported: crystal, dmd, go, java, ldc2, nodejs, php, python, rust_actix, rust_hyper, rust_rocket, rust_tide, rust_warp, scala.

And another program to get the versions of the languages:

    $ cargo run --manifest-path suite/Cargo.toml --bin versions

The same `Makefile` is used to run those programs (but the commands should be run from within the Docker image shell):

 - `make run` (run all tests);
 - `make versions` (get the versions of the languages).

## Usage

### Go

    go run go/main.go

### Crystal

Using [Crystal](https://crystal-lang.org/reference/installation/):

    crystal run --release --no-debug crystal/server.cr

### Rust

Please install [Nightly Rust](https://github.com/rust-lang/rustup.rs#working-with-nightly-rust).

Sample applications use [hyper](https://hyper.rs) HTTP library, [Rocket](https://rocket.rs/), [Tide](https://crates.io/crates/tide), [warp](https://crates.io/crates/warp) and [Actix](https://actix.rs/) web frameworks:

    cargo run --manifest-path rust/hyper/Cargo.toml --release
    cargo run --manifest-path rust/rocket/Cargo.toml --release
    cargo run --manifest-path rust/tide/Cargo.toml --release
    cargo run --manifest-path rust/warp/Cargo.toml --release
    cargo run --manifest-path rust/actix-web/Cargo.toml --release

### D

Two compilers are tested:

 - DMD (a reference D compiler);
 - [LDC](https://github.com/ldc-developers/ldc#installation) (LLVM-based D compiler).
If ldc2 executable is not in path, please use the fully qualified path name.

Uses [vibe.d](https://vibed.org/) framework:

    dub run --root=d --compiler=dmd --build=release --config=dmd
    dub run --root=d --compiler=ldc2 --build=release --config=ldc

### Scala

Uses [Akka](https://akka.io/) toolkit:

    make -C scala clean run

### Node.js

    node nodejs/main.js

### PHP

Uses standalone web server and [Swoole](https://www.swoole.co.uk/) extension:

    php -S 127.0.0.1:3000 php/bare/main.php
    php -c php/swoole/php.ini php/swoole/main.php

### Python

Uses standalone web server and [Twisted](https://twistedmatrix.com/trac/) engine:

    python3 python/main.py
    pypy3 python/twist.py

Please note that CPython has the performance problems running as a standalone server, so we've used PyPy3. To install Twisted please use the pip module:

    pypy3 -m ensurepip
    pypy3 -m pip install twisted

### Java

Uses [Sprint Boot](https://spring.io/projects/spring-boot) project:

    make -C java clean run

# Environment

CPU: Intel(R) Core(TM) i7-10710U, Mem: 16GB LPDDR3 2133MHz

Base Docker image: Debian GNU/Linux bullseye/sid

| Language     | Version                         |
| ------------ | ------------------------------- |
| Crystal      | 0.36.1                          |
| DMD          | v2.095.1                        |
| Go           | go1.16                          |
| Java         | 15.0.2                          |
| LDC          | 1.25.1                          |
| Node.js      | v15.11.0                        |
| PHP          | 7.4.15                          |
| PyPy         | 7.3.3-beta0 for Python 3.7.9    |
| Rust         | 1.52.0-nightly                  |
| Scala        | 2.13.5                          |
