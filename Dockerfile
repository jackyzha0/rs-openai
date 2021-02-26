# Stage 1: build step
FROM rust:latest as builder
WORKDIR "/project/app"
RUN rustup default nightly && rustup update

# copy dependency files
COPY Cargo.toml Cargo.toml
COPY Cargo.lock Cargo.lock

# get user application dependencies
RUN cargo fetch

# copy source code
COPY src ./src

RUN cargo build --release

# Stage 2: actual container
FROM rust:slim
WORKDIR "/project/app"

# get files and built binary from previous image
COPY --from=builder /project/app/target/release/ ./

EXPOSE 8000

ENTRYPOINT ["./openai-client"]