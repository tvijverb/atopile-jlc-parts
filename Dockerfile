ARG VARIANT="bookworm"

FROM rust:slim-${VARIANT} as build
RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
    && apt-get -y install --no-install-recommends sudo libpq-dev pkg-config mold

WORKDIR /app

COPY . .
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 as release
WORKDIR /app
COPY --from=build /app/target/release/atopile-jlc-parts .
EXPOSE 3000
CMD ["./atopile-jlc-parts"]
