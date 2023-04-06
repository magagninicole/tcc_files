FROM rustlang/rust:nightly-alpine

RUN apk update
RUN apk add qemu-system-riscv64 make
RUN rustup target add riscv64imac-unknown-none-elf

WORKDIR /usr/src/strail
COPY . .

ENTRYPOINT ["make", "test"]
