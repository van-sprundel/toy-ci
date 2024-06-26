FROM rust:latest as build
WORKDIR /app

COPY . /app

RUN cargo build

FROM gcr.io/distroless/cc-debian12 as run
COPY --from=build /app/target/debug/merel /

EXPOSE 3000

CMD ["./merel"]

