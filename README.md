# Simple Web Benchmark

A simple web benchmark of Go, Crystal, Rust, D, Scala, Node.js and Crystal.

## Preliminary Results

### MacOS

OS: Mac OS X 10.12.6 (16G29)

Hardware: MacBook Pro (CPU: 2.3 GHz Intel Core i7, Mem: 16 GB 1600 MHz DDR3)

Software: Go 1.9, Rust 1.22.0-nightly, Scala 2.12.3, Node.js 8.6.0, DMD 2.076.0,
LDC 1.4.0, Crystal 0.23.1.

![](suite/results/mac.png?raw=true)

### Windows 10

OS: Microsoft Windows [Version 10.0.15063]

Hardware: Dell XPS (CPU: 2.6 GHz Intel Core i7, Mem: 16 GB 2133 MHz DDR4)

Software: Go 1.9, Rust 1.22.0-nightly, Scala 2.12.3, Node.js 8.6.0, DMD 2.076.0,
LDC 1.4.0, Crystal 0.23.1 (under WSL).

![](suite/results/win.png?raw=true)

### WSL

OS: 4.4.0-43-Microsoft GNU/Linux

Hardware: Dell XPS (CPU: 2.6 GHz Intel Core i7, Mem: 16 GB 2133 MHz DDR4)

Software: Go 1.9, Rust 1.22.0-nightly, Scala 2.12.3, Node.js 8.6.0, DMD 2.076.0,
LDC 1.4.0, Crystal 0.23.1.

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

### Rust

Please install [Nightly Rust](https://doc.rust-lang.org/1.5.0/book/nightly-rust.html).
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
