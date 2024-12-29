FROM ubuntu:24.04 AS builder

WORKDIR /app

RUN apt update
RUN apt install -y curl
RUN apt install -y build-essential
RUN apt install -y pkg-config
RUN apt-get install -y libpq-dev
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

RUN strip target/release/backend

FROM ubuntu:24.04 AS release

RUN apt update
RUN apt install -y curl
RUN apt install -y pkg-config
RUN apt-get install -y libpq-dev
RUN apt install -y libasound2-dev

WORKDIR /app

COPY --from=builder /app/target/release/backend .
EXPOSE 8080
CMD ["./backend"]