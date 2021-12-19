FROM rust:latest as builder

WORKDIR /app

RUN cargo init

COPY ./.cargo .cargo
COPY ./vendor vendor
COPY Cargo.toml Cargo.lock ./
RUN cargo build
RUN cargo clean -p mandelbrot_worker

COPY ./src src

RUN cargo install --path .

FROM gcr.io/distroless/cc-debian11
COPY --from=builder /usr/local/cargo/bin/* /usr/local/bin/

CMD ["mandelbrot_worker"]