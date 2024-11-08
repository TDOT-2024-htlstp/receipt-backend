FROM rust:latest AS build
WORKDIR /app
COPY . .
RUN cargo build --release

FROM rust:latest
WORKDIR /app
COPY --from=build /app/target/release/testing-bon .
EXPOSE 8081
CMD ["./testing-bon"]
