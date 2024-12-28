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
RUN mkdir ./src
# Will be replaced by the actual code
RUN echo 'fn main() { panic!("Dummy Image Called!")}' > ./src/main.rs
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release
COPY ./ .
RUN cargo build --release

EXPOSE 8080
CMD ["./target/release/backend"]