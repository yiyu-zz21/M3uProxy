FROM rust:1.75 AS builder

WORKDIR /app

# 预先拷贝依赖清单以最大化缓存命中
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {println!(\"dummy\");}" > src/main.rs && cargo build --release && rm -rf src

# 拷贝真实源码并编译
COPY src ./src
RUN cargo build --release

FROM debian:bookworm-slim AS runtime

WORKDIR /app

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/m3u_proxy /usr/local/bin/m3u_proxy
COPY Gather.m3u ./Gather.m3u

EXPOSE 8006

CMD ["m3u_proxy"]
