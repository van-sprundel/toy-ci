FROM rust AS watch
WORKDIR /app
COPY . .

RUN cargo install --locked cargo-watch
