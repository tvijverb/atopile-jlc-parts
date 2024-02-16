ARG VARIANT="bookworm"
ARG TARGETPLATFORM
# RUN if [ "$TARGETPLATFORM" = "linux/amd64" ]; then ARCHITECTURE=amd64; elif [ "$TARGETPLATFORM" = "linux/arm/v7" ]; then ARCHITECTURE=arm; elif [ "$TARGETPLATFORM" = "linux/arm64" ]; then ARCHITECTURE=aarch64; else ARCHITECTURE=amd64;

FROM rust:slim-${VARIANT} as build

WORKDIR /jlcparts

RUN  apt-get update && apt-get upgrade -y \
    && apt-get install -y mold wget curl p7zip-full g++ \
    && apt-get clean && rm -rf /var/lib/apt/lists/*

RUN curl -L --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.sh | bash
RUN cargo binstall jlcpcb-to-parquet -y


RUN wget https://yaqwsx.github.io/jlcparts/data/cache.zip https://yaqwsx.github.io/jlcparts/data/cache.z01 \
    https://yaqwsx.github.io/jlcparts/data/cache.z02 https://yaqwsx.github.io/jlcparts/data/cache.z03 \
    https://yaqwsx.github.io/jlcparts/data/cache.z04 https://yaqwsx.github.io/jlcparts/data/cache.z05 \
    https://yaqwsx.github.io/jlcparts/data/cache.z06 https://yaqwsx.github.io/jlcparts/data/cache.z07 \
    https://yaqwsx.github.io/jlcparts/data/cache.z08 https://yaqwsx.github.io/jlcparts/data/cache.z09
RUN 7z x cache.zip

# remove the zip files
RUN rm cache.zip cache.z01 cache.z02 cache.z03 cache.z04 cache.z05 cache.z06 cache.z07 cache.z08 cache.z09

RUN jlcpcb-to-parquet

# COPY ../Cargo.toml ./Cargo.toml
# # Build empty app with downloaded dependencies to produce a stable image layer for next build
# RUN cargo build --release

WORKDIR /app

RUN cp /jlcparts/components.parquet /app/components.parquet

COPY . .
COPY Cargo.toml ./Cargo.toml
RUN cargo build --release

FROM debian:${VARIANT}-slim
COPY --from=build /app/target/release/atopile-jlc-parts .
COPY --from=build /app/components.parquet .
EXPOSE 3000
CMD ["./atopile-jlc-parts"]
