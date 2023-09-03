FROM rust:latest
WORKDIR /app
COPY . .

ENV ROCKET_ADDRESS=0.0.0.0

EXPOSE 8000
CMD ["cargo", "run", "--release"]