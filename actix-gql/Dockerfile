FROM rust:1.45-slim as build

WORKDIR /home/build

COPY . .

RUN cargo build --release

# Release image
FROM debian:10-slim

WORKDIR /home/dv4all

COPY --from=build /home/build/target/release/actix-gql /home/dv4all/
COPY ./static/ /home/dv4all/static

CMD ["/home/dv4all/actix-gql"]



