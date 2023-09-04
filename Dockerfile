FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:latest
WORKDIR /app
COPY --from=builder /app/.env .env
COPY --from=builder /app/target/release/everyonetoday-api app
RUN chmod +x app
ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000
CMD ["sh", "-c", "./app"]