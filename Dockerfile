ARG VARIANT="bookworm"
ARG TARGETPLATFORM
# RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then ARCHITECTURE=amd64; elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then ARCHITECTURE=arm; elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then ARCHITECTURE=aarch64; else ARCHITECTURE=amd64;

FROM rust:slim-${VARIANT} as build

WORKDIR /jlcparts

RUN  apt-get update && apt-get upgrade -y \
    && apt-get install -y mold wget curl p7zip-full g++ \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN curl -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall jlcpcb-to-parquet -y || (echo "cargo binstall failed, fallback on cargo install"; cargo install --git https://github.com/tvijverb/jlcpcb-to-parquet; exit 0)


RUN wget https://yaqwsx.github.io/jlcparts/data/cache.zip https://yaqwsx.github.io/jlcparts/data/cache.z01 \
    https://yaqwsx.github.io/jlcparts/data/cache.z02 https://yaqwsx.github.io/jlcparts/data/cache.z03 \
    https://yaqwsx.github.io/jlcparts/data/cache.z04 https://yaqwsx.github.io/jlcparts/data/cache.z05 \
    https://yaqwsx.github.io/jlcparts/data/cache.z06 https://yaqwsx.github.io/jlcparts/data/cache.z07 \
    https://yaqwsx.github.io/jlcparts/data/cache.z08 https://yaqwsx.github.io/jlcparts/data/cache.z09
RUN 7z x cache.zip

# remove the zip files
RUN rm cache.zip cache.z01 cache.z02 cache.z03 cache.z04 cache.z05 cache.z06 cache.z07 cache.z08 cache.z09

RUN jlcpcb-to-parquet || /usr/local/cargo/bin/jlcpcb-to-parquet; exit 0

# COPY ../Cargo.toml ./Cargo.toml
# # Build empty app with downloaded dependencies to produce a stable image layer for next build
# RUN cargo build --release

WORKDIR /app

RUN cp /jlcparts/components.parquet /app/components.parquet
RUN cp /jlcparts/resistors.parquet /app/resistors.parquet
RUN cp /jlcparts/capacitors.parquet /app/capacitors.parquet
RUN cp /jlcparts/inductors.parquet /app/inductors.parquet

COPY . .
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM debian:${VARIANT}-slim
COPY --from=build /app/target/release/atopile-jlc-parts .
COPY --from=build /app/components.parquet .
COPY --from=build /app/resistors.parquet .
COPY --from=build /app/capacitors.parquet .
COPY --from=build /app/inductors.parquet .
EXPOSE 3000
CMD ["./atopile-jlc-parts"]
