from debian:9

RUN apt-get update -q && apt-get upgrade -yyq
RUN apt-get install -yyq \
  autoconf \
  build-essential \
  curl \
  clang-3.9 \
  git-core \
  libtool

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

RUN git clone https://github.com/zzeroo/libmodbus-rs.git
RUN cd libmodbus-rs && \
  . ~/.profile && \
  cargo test
