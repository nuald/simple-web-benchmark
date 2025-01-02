FROM debian:trixie-20241016-slim

ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=1 \
    DEBIAN_FRONTEND=noninteractive

RUN apt-get update && apt-get install -y --no-install-recommends \
    build-essential \
    ca-certificates \
    cmake \
    curl \
    fontconfig \
    libfontconfig-dev \
    git \
    libboost-dev \
    libevent-dev \
    libexpat1-dev \
    libfreetype-dev \
    libssl-dev \
    lsof \
    php \
    php-dev \
    php-pear \
    pkg-config \
    python3 \
    python3-twisted \
    xz-utils \
    zlib1g-dev \
    && rm -rf /var/lib/apt/lists/*

RUN update-ca-certificates \
    && pecl channel-update pecl.php.net \
    && pecl install openswoole-22.1.2

RUN useradd -m dev
USER dev
RUN mkdir /home/dev/bin
WORKDIR /home/dev/bin
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# https://jdk.java.net/
ARG JDK=23.0.1
RUN curl \
    https://download.java.net/java/GA/jdk${JDK}/c28985cbf10d4e648e4004050f8781aa/11/GPL/openjdk-${JDK}_linux-x64_bin.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/jdk-$JDK/bin:${PATH}"

# https://github.com/crystal-lang/crystal/releases
ARG CRYSTAL=crystal-1.14.0-1
RUN curl -L \
    https://github.com/crystal-lang/crystal/releases/download/1.14.0/$CRYSTAL-linux-x86_64.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/$CRYSTAL/bin:${PATH}"

# https://github.com/ldc-developers/ldc/releases
ARG D=1.40.0
RUN curl -L \
    https://github.com/ldc-developers/ldc/releases/download/v$D/ldc2-$D-linux-x86_64.tar.xz \
    | tar -xz
ENV PATH="/home/dev/bin/ldc2-linux-x86_64/bin:${PATH}"

# https://golang.org/dl/
RUN curl -L \
    https://go.dev/dl/go1.23.3.linux-amd64.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/go/bin/:${PATH}"

# https://www.scala-lang.org/download/
ARG SCALA=3.5.2
RUN curl -L \
    https://github.com/lampepfl/dotty/releases/download/$SCALA/scala3-$SCALA.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/scala3-$SCALA/bin/:${PATH}"

# https://nodejs.org/en/download/current/
ARG NODE=v23.2.0
RUN curl \
    https://nodejs.org/dist/$NODE/node-$NODE-linux-x64.tar.xz \
    | tar -xJ
ENV PATH="/home/dev/bin/node-$NODE-linux-x64/bin/:${PATH}"

# https://pypy.org/download.html
ARG PYPY=pypy3.10-v7.3.17-linux64
RUN curl \
    https://downloads.python.org/pypy/$PYPY.tar.bz2 \
    | tar -xj \
    && rm /home/dev/bin/$PYPY/bin/python*
ENV PATH="/home/dev/bin/$PYPY/bin:${PATH}"

RUN pypy3 -m ensurepip \
    && pypy3 -m pip install --upgrade pip \
    && pypy3 -m pip install twisted

# https://www.rust-lang.org/tools/install
ENV CARGO_HOME="/home/dev/bin/.cargo" PATH="/home/dev/bin/.cargo/bin:${PATH}"
RUN curl https://sh.rustup.rs | sh -s -- -y --default-toolchain nightly

RUN curl -Lo /home/dev/bin/coursier https://git.io/coursier-cli \
    && chmod +x /home/dev/bin/coursier
ENV PATH="/home/dev/bin/:${PATH}"

ENV GOPATH="/home/dev/.go" PATH="/home/dev/.go/bin:${PATH}"
RUN go install github.com/rakyll/hey@latest

RUN crystal --version && echo "---" \
    && ldc2 --version && echo "---" \
    && go version && echo "---" \
    && rustc --version && echo "---" \
    && scala --version && echo "---" \
    && node -e "console.log('Nodejs ' + process.version)" && echo "---" \
    && python3 --version && echo "---" \
    && java --version && echo "---" \
    && php --version && echo "---" \
    echo " END"

WORKDIR /src
ENTRYPOINT ["/bin/bash"]

RUN cat /etc/os-release
