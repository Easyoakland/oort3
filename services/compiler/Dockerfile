# fetcher
FROM rust:1.77.2-slim-bookworm as fetcher
WORKDIR /usr/src/app
RUN cargo init --bin dummy
COPY Cargo.lock.user Cargo.lock
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  --mount=type=cache,target=/usr/src/app/target,id=services_target,sharing=locked \
  cd dummy && cargo fetch && cp -a /usr/local/cargo/registry /registry

# builder
FROM rust:1.77.2-slim-bookworm as builder
WORKDIR /usr/src/app
COPY . .
ARG OORT_CODE_ENCRYPTION_SECRET
RUN \
  --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=cache,target=/usr/local/cargo/git \
  --mount=type=cache,target=/usr/src/app/target,id=services_target,sharing=locked \
  cd services/compiler && cargo install --profile service --locked --path .

# service
FROM rust:1.77.2-slim-bookworm
RUN apt-get -y update && apt-get -y install curl xz-utils && rm -rf /var/lib/apt/lists/*
RUN rustup target add wasm32-unknown-unknown
RUN rustup component add rustfmt
RUN mkdir /opt/zig && curl https://ziglang.org/download/0.11.0/zig-linux-x86_64-0.11.0.tar.xz | tar -xJf - -C /opt/zig --strip-components=1
RUN ln -s /opt/zig/zig /usr/bin/
RUN useradd -m app
USER app:1000
WORKDIR /home/app
COPY --from=builder /usr/local/cargo/bin/oort_compiler_service /usr/local/bin/oort_compiler_service
ENV PORT 8080
ENV RUST_LOG info
RUN oort_compiler_service --prepare
CMD ["oort_compiler_service"]
