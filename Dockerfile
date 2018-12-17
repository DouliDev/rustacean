FROM rust:1.30.1-slim

RUN mkdir -p /usr/share/man/man1

RUN apt-get update -y && apt-get install -y libssl-dev \
    pkg-config \
    python3 \
    g++ \
    nodejs \
    php-cli \
    mono-devel \
    lua5.3 \
    lua-socket \
    lua-sec \
    openjdk-8-jdk 

COPY ./ /home

WORKDIR /home

RUN cargo build

CMD ["cargo", "run"]
