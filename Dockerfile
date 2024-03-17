ARG VARIANT="bookworm"

FROM rust:slim-${VARIANT} as build

WORKDIR /app

COPY . .
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM debian:${VARIANT}-slim
COPY --from=build /app/target/release/atopile-jlc-parts .
EXPOSE 3000
CMD ["./atopile-jlc-parts"]
