FROM rust:1.54
WORKDIR /usr/src/multiplayer-server
COPY . .
RUN cargo install --path .

ENV RUST_LOG=trace

CMD ["multiplayer-server"]