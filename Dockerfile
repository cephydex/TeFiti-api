FROM rust:1.78

WORKDIR /app
COPY . .

# Build your program for release
RUN cargo build --release

# Run the binary
CMD ["/app/target/release/svctrackerapi"]