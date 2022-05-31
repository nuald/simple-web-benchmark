FROM debian:testing

ENV APT_KEY_DONT_WARN_ON_DANGEROUS_USAGE=1 \
    DEBIAN_FRONTEND=noninteractive
COPY ./apt.pkgs /tmp/apt.pkgs
RUN apt-get update && xargs apt-get install -y < /tmp/apt.pkgs \
    && pecl install swoole \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m dev

USER dev
RUN mkdir /home/dev/bin
WORKDIR /home/dev/bin
SHELL ["/bin/bash", "-o", "pipefail", "-c"]

# https://jdk.java.net/
ARG JDK=18.0.1.1
RUN curl \
    https://download.java.net/java/GA/jdk${JDK}/65ae32619e2f40f3a9af3af1851d6e19/2/GPL/openjdk-${JDK}_linux-x64_bin.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/jdk-$JDK/bin:${PATH}"

# https://github.com/crystal-lang/crystal/releases
ARG CRYSTAL=crystal-1.4.1-1
RUN curl -L \
	https://github.com/crystal-lang/crystal/releases/download/1.4.1/$CRYSTAL-linux-x86_64.tar.gz \
	| tar -xz
ENV PATH="/home/dev/bin/$CRYSTAL/bin:${PATH}"

# https://github.com/ldc-developers/ldc/releases
ARG LDC=ldc2-1.29.0-linux-x86_64
RUN curl -L \
	https://github.com/ldc-developers/ldc/releases/download/v1.29.0/$LDC.tar.xz \
	| tar -xJ
ENV PATH="/home/dev/bin/$LDC/bin/:${PATH}"

# https://golang.org/dl/
RUN curl -L \
    https://go.dev/dl/go1.18.2.linux-amd64.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/go/bin/:${PATH}"

# https://dlang.org/download.html
RUN curl \
    https://s3.us-west-2.amazonaws.com/downloads.dlang.org/releases/2022/dmd.2.100.0.linux.tar.xz \
    | tar -xJ
ENV PATH="/home/dev/bin/dmd2/linux/bin64/:${PATH}"

# https://www.scala-lang.org/download/
ARG SCALA=3.1.2
RUN curl -L \
    https://github.com/lampepfl/dotty/releases/download/$SCALA/scala3-$SCALA.tar.gz \
    | tar -xz
ENV PATH="/home/dev/bin/scala3-$SCALA/bin/:${PATH}"

# https://nodejs.org/en/download/current/
ARG NODE=v18.2.0
RUN curl \
	https://nodejs.org/dist/$NODE/node-$NODE-linux-x64.tar.xz \
	| tar -xJ
ENV PATH="/home/dev/bin/node-$NODE-linux-x64/bin/:${PATH}"

# https://pypy.org/download.html
ARG PYPY=pypy3.9-v7.3.9-linux64
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

ENV GEM_HOME="/home/dev/bundle"

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
ENTRYPOINT ["/bin/bash"]

RUN cat /etc/os-release
