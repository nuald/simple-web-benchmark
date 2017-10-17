FROM alpine:3.6

RUN apk add --update ca-certificates wget
RUN update-ca-certificates

# LLVM and other required packages are in the community repo
RUN sed -i 's/#(.*v3.6\/community.*)/\1/' /etc/apk/repositories

# Adding crystal-alpine repo
RUN echo "http://public.portalier.com/alpine/testing" >> /etc/apk/repositories
RUN wget "https://raw.githubusercontent.com/ysbaddaden/crystal-alpine/master/conf/julien%40portalier.com-56dab02e.rsa.pub" \
    -O /etc/apk/keys/julien@portalier.com-56dab02e.rsa.pub

RUN apk add --update git go libc-dev openjdk8 crystal nodejs rust bash libc6-compat

# Install Hey
RUN go get github.com/rakyll/hey
RUN ln -sf /root/go/bin/hey /usr/bin/hey

# Install Conscript
RUN wget https://raw.githubusercontent.com/foundweekends/conscript/master/setup.sh
RUN sh setup.sh
RUN ln -sf /root/.conscript/bin/cs /usr/bin/cs

# Install scalas
RUN cs sbt/sbt
RUN ln -sf /root/.conscript/bin/scalas /usr/bin/scalas

# Install gradle
RUN wget https://services.gradle.org/distributions/gradle-4.2.1-bin.zip -O /root/gradle-4.2.1-bin.zip
RUN unzip /root/gradle-4.2.1-bin.zip -d /root/
RUN rm /root/gradle-4.2.1-bin.zip
RUN ln -sf /root/gradle-4.2.1/bin/gradle /usr/bin/gradle

RUN rm -rf /var/cache/apk/*

# Check out the benchmark
ENV HEAD=d0c392ce9dfd34937db58d08445f686f45b13c21
RUN git clone -n https://github.com/nuald/simple-web-benchmark.git /var/swb && \
    cd /var/swb && \
    git checkout $HEAD

WORKDIR "/var/swb"
ENTRYPOINT []
LABEL org.mobyproject.config='{"binds": ["/etc/resolv.conf:/etc/resolv.conf"]}'
