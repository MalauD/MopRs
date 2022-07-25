FROM lukemathwalker/cargo-chef:latest-rust-1.56.0 AS chef
WORKDIR app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
RUN cargo build --release
# We do not need the Rust toolchain to run the binary!

FROM node AS npmbuilder
COPY . .
RUN npm install
RUN npx webpack --mode production

FROM debian:buster-slim AS runtime
WORKDIR app
COPY --from=builder /app/target/release/mop-rs .
RUN mkdir static
COPY --from=npmbuilder /static ./static
ENTRYPOINT ["./mop-rs"]