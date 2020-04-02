FROM debian:testing

RUN apt update \
    && apt install -y wget xz-utils build-essential pkg-config cmake lsof fontconfig \
       libpcre3-dev libevent-dev libssl-dev zlib1g-dev libfreetype-dev libexpat1-dev \
       python3 pypy3 python3-twisted php php-dev \
    && pecl install swoole \
    && rm -rf /var/lib/apt/lists/* /tmp/*

WORKDIR /opt

# https://jdk.java.net/
ARG JDK=14
RUN wget --progress=dot:giga -O - \
    https://download.java.net/java/GA/jdk14/076bab302c7b4508975440c56f6cc26a/36/GPL/openjdk-${JDK}_linux-x64_bin.tar.gz \
    | tar -xz
ENV PATH="/opt/jdk-$JDK/bin:${PATH}"

# https://github.com/crystal-lang/crystal/releases
ARG CRYSTAL=crystal-0.33.0-1
RUN wget --progress=dot:giga -O - \
    https://github.com/crystal-lang/crystal/releases/download/0.33.0/$CRYSTAL-linux-x86_64.tar.gz \
    | tar -xz
ENV PATH="/opt/$CRYSTAL/bin:${PATH}"

# https://github.com/ldc-developers/ldc/releases
ARG LDC=ldc2-1.20.1-linux-x86_64
RUN wget --progress=dot:giga -O - \
    https://github.com/ldc-developers/ldc/releases/download/v1.20.1/$LDC.tar.xz \
    | tar -xJ
ENV PATH="/opt/$LDC/bin/:${PATH}"

# https://golang.org/dl/
RUN wget --progress=dot:giga -O - \
    https://dl.google.com/go/go1.14.1.linux-amd64.tar.gz \
    | tar -xz
ENV PATH="/opt/go/bin/:${PATH}"

# https://dlang.org/download.html
RUN wget --progress=dot:giga -O - \
    http://downloads.dlang.org/releases/2.x/2.091.0/dmd.2.091.0.linux.tar.xz \
    | tar -xJ
ENV PATH="/opt/dmd2/linux/bin64/:${PATH}"

# https://www.scala-lang.org/download/
ARG SCALA=2.13.1
RUN wget --progress=dot:giga -O - \
    https://downloads.lightbend.com/scala/$SCALA/scala-$SCALA.tgz \
    | tar -xz
ENV PATH="/opt/scala-$SCALA/bin/:${PATH}"

# https://nodejs.org/en/download/current/
ARG NODE=v13.12.0
RUN wget --progress=dot:giga -O - \
    https://nodejs.org/dist/$NODE/node-$NODE-linux-x64.tar.xz \
    | tar -xJ
ENV PATH="/opt/node-$NODE-linux-x64/bin/:${PATH}"

# https://www.rust-lang.org/tools/install
ENV CARGO_HOME="/opt/.cargo" PATH="/opt/.cargo/bin:${PATH}"
RUN wget --progress=dot:giga --secure-protocol=TLSv1_2 -O - https://sh.rustup.rs \
    | sh -s -- -y --default-toolchain nightly

RUN wget -O /usr/local/bin/coursier https://git.io/coursier-cli \
    && chmod +x /usr/local/bin/coursier

# https://github.com/rakyll/hey
RUN wget --progress=dot:giga \
    https://storage.googleapis.com/hey-release/hey_linux_amd64 \
    && chmod +x hey_linux_amd64 \
    && ln -s /opt/hey_linux_amd64 /usr/bin/hey

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
