# Simple Web Benchmark

A simple web benchmark of Crystal, D, Go, Java, Node.js, PHP, Python, Rust and  Scala.

## Results

OS: Ubuntu 18.04.3 LTS

Hardware: CPU: 2.6 GHz Intel Core i7, Mem: 8 GB 1333 MHz DDR3

![](suite/results/lin.png?raw=true)

## Testing

The stats gathered by the [hey](https://github.com/rakyll/hey) tool (please run it twice for
the JIT optimizations where it's applicable):

    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/"
    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/greeting/hello"


### Using Docker

Build the image:

    $ docker build suite/ -t simple-web-benchmark

Enter the shell in the image:

    $ docker run -it --rm --name test -v $(pwd):/src --network="host" simple-web-benchmark

### Automation

Please use the Scala script to run all the tests automatically (requires [Ammonite](https://ammonite.io/)).

    Usage: amm suite/run.scala [options] <lang>...

      -o, --out <file>  image file to generate (result.png by default)
      --verbose         verbose execution output
      <lang>...         languages to test ('all' for all)

    The following languages are supported: rust_hyper, rust_rocket, crystal, nodejs, go, scala, dmd, ldc2.

## Usage

### Go

    go run go/main.go

### Crystal

Using [Crystal](https://crystal-lang.org/reference/installation/):

    crystal run --release --no-debug crystal/server.cr

### Rust

Please install [Nightly Rust](https://github.com/rust-lang/rustup.rs#working-with-nightly-rust).

Sample applications use [hyper](https://hyper.rs) HTTP library and [Rocket](https://rocket.rs/) web framework:

    cargo run --manifest-path rust/hyper/Cargo.toml --release
    cargo run --manifest-path rust/rocket/Cargo.toml --release

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

# Software

Software: Crystal 0.31.1, DMD 2.088.1, LDC 1.18.0, Go 1.13.4, Java (OpenJDK) 11.0.4, Node.js 12.13.0, PHP 7.2.24, Python 3.6.8 (PyPy 7.1.1), Rust 1.40.0-nightly, Scala 2.13.1.
