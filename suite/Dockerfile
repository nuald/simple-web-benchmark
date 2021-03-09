FROM debian:testing

RUN apt-get update \
    && apt-get install -y wget xz-utils build-essential pkg-config cmake lsof fontconfig \
       libpcre3-dev libevent-dev libssl-dev zlib1g-dev libfreetype-dev libexpat1-dev \
       python3 python3-twisted php php-dev git \
    && pecl install swoole \
    && rm -rf /var/lib/apt/lists/* /tmp/*

WORKDIR /opt

# https://jdk.java.net/
ARG JDK=15.0.2
RUN wget --progress=dot:giga -O - \
    https://download.java.net/java/GA/jdk15.0.2/0d1cfde4252546c6931946de8db48ee2/7/GPL/openjdk-${JDK}_linux-x64_bin.tar.gz \
    | tar -xz
ENV PATH="/opt/jdk-$JDK/bin:${PATH}"

# https://github.com/crystal-lang/crystal/releases
ARG CRYSTAL=crystal-0.36.1-1
RUN wget --progress=dot:giga -O - \
	https://github.com/crystal-lang/crystal/releases/download/0.36.1/$CRYSTAL-linux-x86_64.tar.gz \
	| tar -xz
ENV PATH="/opt/$CRYSTAL/bin:${PATH}"

# https://github.com/ldc-developers/ldc/releases
ARG LDC=ldc2-1.25.1-linux-x86_64
RUN wget --progress=dot:giga -O - \
	https://github.com/ldc-developers/ldc/releases/download/v1.25.1/$LDC.tar.xz \
	| tar -xJ
ENV PATH="/opt/$LDC/bin/:${PATH}"

# https://golang.org/dl/
RUN wget --progress=dot:giga -O - \
        https://golang.org/dl/go1.16.linux-amd64.tar.gz \
	| tar -xz
ENV PATH="/opt/go/bin/:${PATH}"

# https://dlang.org/download.html
RUN wget --progress=dot:giga -O - \
    https://s3.us-west-2.amazonaws.com/downloads.dlang.org/releases/2021/dmd.2.095.1.linux.tar.xz \
    | tar -xJ
ENV PATH="/opt/dmd2/linux/bin64/:${PATH}"

# https://www.scala-lang.org/download/
ARG SCALA=2.13.5
RUN wget --progress=dot:giga -O - \
	https://downloads.lightbend.com/scala/$SCALA/scala-$SCALA.tgz \
	| tar -xz
ENV PATH="/opt/scala-$SCALA/bin/:${PATH}"

# https://nodejs.org/en/download/current/
ARG NODE=v15.11.0
RUN wget --progress=dot:giga -O - \
	https://nodejs.org/dist/$NODE/node-$NODE-linux-x64.tar.xz \
	| tar -xJ
ENV PATH="/opt/node-$NODE-linux-x64/bin/:${PATH}"

# https://pypy.org/download.html
ARG PYPY=pypy3.7-v7.3.3-linux64
RUN wget --progress=dot:giga -O - \
        https://downloads.python.org/pypy/$PYPY.tar.bz2 \
	| tar -xj
ENV PATH="/opt/$PYPY/bin:${PATH}"
RUN pypy3 -m ensurepip && pypy3 -m pip install twisted

# https://www.rust-lang.org/tools/install
ENV CARGO_HOME="/opt/.cargo" PATH="/opt/.cargo/bin:${PATH}"
RUN wget -O - https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

RUN wget -O /usr/local/bin/coursier https://git.io/coursier-cli \
    && chmod +x /usr/local/bin/coursier

ENV GOPATH="/opt/.go" PATH="/opt/.go/bin:${PATH}"
RUN go get github.com/rakyll/hey

RUN crystal --version && echo "---" \
    && go version && echo "---" \
    && dmd --version && echo "---" \
    && ldc2 --version && echo "---" \
    && rustc --version && echo "---" \
    && scala --version && echo "---" \
    && node -e "console.log('Nodejs ' + process.version)" && echo "---" \
    && python3 --version && echo "---" \
    && java --version && echo "---" \
    && php --version && echo "---" \
    echo " END"

WORKDIR /src
ENTRYPOINT bash

RUN cat /etc/os-release
