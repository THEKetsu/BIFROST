FROM rust
WORKDIR /src
COPY ./ ./
RUN cargo build --release
CMD ["/bin/docker-program"]