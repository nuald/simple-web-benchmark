FROM alpine:3.6

RUN apk add --update ca-certificates wget
RUN update-ca-certificates

# LLVM and other required packages are in the community repo
RUN sed -i 's/#(.*v3.6\/community.*)/\1/' /etc/apk/repositories

# Adding crystal-alpine repo
RUN echo "http://public.portalier.com/alpine/testing" >> /etc/apk/repositories
RUN wget "https://raw.githubusercontent.com/ysbaddaden/crystal-alpine/master/conf/julien%40portalier.com-56dab02e.rsa.pub" \
    -O /etc/apk/keys/julien@portalier.com-56dab02e.rsa.pub

RUN apk add --update git go libc-dev openjdk8 crystal nodejs rust bash

RUN git clone https://github.com/nuald/simple-web-benchmark.git /var/swb

# Install Hey
RUN go get github.com/rakyll/hey
RUN ln -sf /root/go/bin/hey /usr/bin/hey

# Install Conscript
RUN wget https://raw.githubusercontent.com/foundweekends/conscript/master/setup.sh
RUN sh setup.sh
RUN ln -sf /root/.conscript/bin/cs /usr/bin/cs

# Install sbt
RUN cs sbt/sbt
RUN ln -sf /root/.conscript/bin/xsbt /usr/bin/sbt
RUN ln -sf /root/.conscript/bin/scalas /usr/bin/scalas

RUN rm -rf /var/cache/apk/*

WORKDIR "/var/swb/suite"
ENTRYPOINT ["/usr/bin/scalas", "run.scala"]
LABEL org.mobyproject.config='{"binds": ["/etc/resolv.conf:/etc/resolv.conf"]}'
