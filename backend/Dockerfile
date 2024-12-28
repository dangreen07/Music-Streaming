FROM ubuntu:22.04

WORKDIR /usr/server/backend

RUN apt update
RUN apt-get install -y libpq-dev
RUN apt install -y curl
RUN apt install -y build-essential
RUN apt install -y pkg-config
RUN apt install -y libasound2-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"
COPY ./ .
RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/backend"]