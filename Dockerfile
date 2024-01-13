FROM rust:1.75 as build

# create a new empty shell project
RUN USER=root cargo new --bin vcard-downloader
WORKDIR /vcard-downloader

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

# this build step will cache your dependencies
RUN cargo build --release
RUN rm src/*.rs

# copy your source tree
COPY ./src ./src

# build for release
RUN rm ./target/release/deps/vcard_downloader*
RUN cargo build --release

# our final base
FROM debian:bookworm-slim

RUN apt-get update && apt-get install sqlite3 -y

WORKDIR /vcard-downloader

# copy the build artifact from the build stage
COPY --from=build /vcard-downloader/target/release/vcard-downloader .
COPY ./public/* ./public/

ENV BIND=0.0.0.0:5000
ENV DATABASE_URL=file:db/card_data.db3

# set the startup command to run your binary
CMD ["./vcard-downloader"]
