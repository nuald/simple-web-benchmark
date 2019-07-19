# Simple Web Benchmark

A simple web benchmark of Crystal, D, Go, Java, Node.js, PHP, Python, Rust and  Scala.

## Results

### Linux

OS: Ubuntu 18.04.2 LTS

Hardware: CPU: 2.6 GHz Intel Core i7, Mem: 8 GB 1333 MHz DDR3

Software: Crystal 0.29.0, DMD 2.087.0, LDC 1.8.0, Go 1.10.4, Java (OpenJDK) 11.0.3, Node.js 12.6.0, PHP 7.2.19, Rust 1.38.0-nightly, Scala 2.12.8.

![](suite/results/lin.png?raw=true)

### MacOS

OS: Mac OS X 10.14.5

Hardware: MacBook Pro (CPU: 2.9 GHz Intel Core i7, Mem: 16 GB 2133 MHz DDR3)

Software: Go 1.12.7, Rust 1.38.0-nightly, Scala 2.12.8, Node.js 12.6.0, DMD 2.087.0, LDC 1.16.0, Crystal 0.29.0, PHP 7.3.7, Java (SE) 1.8.0.

![](suite/results/mac.png?raw=true)

### Windows 10

OS: Microsoft Windows [Version 10.0.17763]

Hardware: Dell XPS (CPU: 2.6 GHz Intel Core i7, Mem: 16 GB 2133 MHz DDR4)

Software: Go 1.12.7, Rust 1.38.0-nightly, Scala 2.12.8, Node.js 12.6.0, DMD 2.087.0,
LDC 1.16.0, Java (OpenJDK) 12.0.2.

![](suite/results/win.png?raw=true)

### WSL

OS: Ubuntu 16.04.6 LTS

Hardware: Dell XPS (CPU: 2.6 GHz Intel Core i7, Mem: 16 GB 2133 MHz DDR4)

Software: Go 1.9.4, Rust 1.38.0-nightly, Scala 2.12.8, Node.js 9.11.2, DMD 2.087.0,
LDC 1.7.0, Crystal 0.29.0, PHP 7.0.33, Java (OpenJDK) 1.8.0.

![](suite/results/wsl.png?raw=true)

## Testing

The stats gathered by the [hey](https://github.com/rakyll/hey) tool (please run it twice for
the JIT optimizations where it's applicable):

    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/"
    hey -n 50000 -c 256 -t 10 "http://127.0.0.1:3000/greeting/hello"

### MacOS Note

By default, MacOS has low limits on the number of concurrent connections, so
few kernel parameters tweaks may be required:

    sudo sysctl -w kern.ipc.somaxconn=12000
    sudo sysctl -w kern.maxfilesperproc=1048576
    sudo sysctl -w kern.maxfiles=1148576

### Automation

Please use the Scala script
(using [sbt Script runner](http://www.scala-sbt.org/1.x/docs/Scripts.html#sbt+Script+runner))
to run all the tests automatically.

    Usage: scalas suite/run.scala [options] <lang>...

      -o, --out <file>  image file to generate (result.png by default)
      --verbose         verbose execution output
      <lang>...         languages to test ('all' for all)

    The following languages are supported: rust_hyper, rust_rocket, crystal, nodejs, go, scala, dmd, ldc2.

## Usage

### Go

    go run go/main.go

### Crystal

Using [Crystal](https://crystal-lang.org/docs/installation/):

    crystal run --release --no-debug crystal/server.cr

*Alpine Linux note: please use [crystal-alpine](https://github.com/ysbaddaden/crystal-alpine) packages.*

*macOS note: linking with OpenSSL may require [PKG_CONFIG_PATH changes](https://github.com/crystal-lang/crystal/issues/4745).*

### Rust

Please install [Nightly Rust](https://github.com/rust-lang-nursery/rustup.rs#working-with-nightly-rust).
Windows also requires [MinGW](https://github.com/rust-lang/rust#mingw)
compiler toolchain with `mingw-w64-x86_64-gcc` installed.

Sample applications use [hyper](https://hyper.rs) HTTP library and [Rocket](https://rocket.rs/) web framework:

    cargo run --manifest-path rust/hyper/Cargo.toml --release
    cargo run --manifest-path rust/rocket/Cargo.toml --release

### D

Two compilers are tested:

 - DMD (a reference D compiler);
 - [LDC](https://github.com/ldc-developers/ldc#installation) (LLVM-based D compiler).
If ldc2 executable is not in path, please use the fully qualified path name.

Uses [vibe.d](http://vibed.org) framework:

    dub run --root=d --compiler=dmd --build=release --config=dmd
    dub run --root=d --compiler=ldc2 --build=release --config=ldc

### Scala

Uses [Akka](http://akka.io) toolkit:

    gradle -p scala run --info

### Node.js

    node nodejs/main.js

### PHP

Uses standalone web server and [Swoole](https://www.swoole.co.uk/) extension:

    php -S 127.0.0.1:3000 php/bare/main.php
    php -c php/swoole/php.ini php/swoole/main.php

### Python

    python3 python/main.py

### Java

Uses [Sprint Boot](https://projects.spring.io/spring-boot/) project:

    gradle -p java build
    java -jar -Dserver.port=3000 java/build/libs/java-0.0.1-SNAPSHOT.jar
