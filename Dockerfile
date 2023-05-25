FROM node:16-alpine as frontend
WORKDIR /frontend

COPY /frontend .
RUN npm i
RUN npm run generate


# Rust Backend
# Using cargo-chef to cache dependencies and increase development time

FROM rust:1.69 AS rust_base
ENV CARGO_REGISTRIES_CRATES_IO_PROTOCOL=sparse
WORKDIR /app
RUN cargo install cargo-chef --locked

FROM rust_base AS planner
COPY /backend .
RUN cargo chef prepare --recipe-path recipe.json

FROM rust_base AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY /backend .
RUN cargo build --release

FROM debian:bullseye-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/sponsormanager /usr/local/bin
COPY --from=frontend /frontend/dist dist/

ENTRYPOINT ["/usr/local/bin/sponsormanager"]
