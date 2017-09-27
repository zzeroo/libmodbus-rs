from base/archlinux

RUN pacman -Syu --noconfirm
RUN pacman -S --noconfirm \
  autoconf \
  clang39 \
  git \
  libtool \
  make
  
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain nightly -y

RUN git clone https://github.com/zzeroo/libmodbus-rs.git
RUN cd libmodbus-rs && \
  . ~/.profile && \
  cargo test
