FROM rust:1.86-slim

WORKDIR /app

RUN apt-get update && apt-get install -y \
    libpq-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-watch

RUN cargo install diesel_cli --no-default-features --features postgres

CMD ["cargo", "watch", "-x", "run"]
