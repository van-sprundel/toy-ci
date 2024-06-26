FROM rust:latest as build
WORKDIR /app

COPY . /app

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12 as run
COPY --from=build /app/target/release/merel /

EXPOSE 3000

CMD ["./merel"]

