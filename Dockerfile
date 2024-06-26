FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json

# add --release for release build, not needed now.
RUN cargo chef cook --recipe-path recipe.json 

RUN cargo install --locked cargo-watch
