FROM node AS npmbuilder
COPY . .
RUN npm install
RUN npx webpack --mode production

FROM rust:1.72.0-slim
WORKDIR app
RUN mkdir static
COPY . .
COPY --from=npmbuilder /static ./static
RUN apt-get update && apt-get install -y ca-certificates pkg-config libssl-dev && update-ca-certificates
RUN cargo build --release
ENTRYPOINT ["./target/release/mop-rs"]
