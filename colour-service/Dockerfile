FROM rust:1.50.0-slim-buster as build-env
WORKDIR /app
ADD . /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev libpq-dev
RUN cargo build --release

FROM debian:buster-slim
COPY --from=build-env /app/target/release/colour-service /
USER nobody
CMD ["./colour-service"]