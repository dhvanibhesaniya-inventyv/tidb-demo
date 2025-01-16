FROM rust:1.73.0-bullseye AS builder

# Create source and release directory
RUN mkdir -p /usr/src/app &&  mkdir -p /usr/local/bin/app

# Set working directory to ./app
WORKDIR /usr/src/app

# Copy source code
COPY . .

# update packge and install code dependencies
RUN apt update && apt upgrade -y

RUN apt install -y cmake libclang-dev

RUN apt install -y protobuf-compiler libprotobuf-dev


# Build RUST application (this command will generate executable file)
RUN cargo install  --path . --target-dir target --profile release


FROM rust:1.73.0-slim-bullseye


# Create release directory
RUN mkdir -p /usr/local/bin/app

# Set working directory to release (exicutable file here)
WORKDIR /usr/local/bin/app

# Copy the built binary from the builder stage
COPY --from=builder /usr/src/app /usr/local/bin/app/

COPY --from=builder /usr/src/app/resources /usr/local/bin/app/resources


# This Command will execute the executable file
CMD ["/usr/local/bin/app/target/release/datalayer-tidb"]