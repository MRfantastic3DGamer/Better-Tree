FROM rust:latest

WORKDIR /app
COPY . .

RUN cargo install --path .

ENTRYPOINT ["better-tree"]

