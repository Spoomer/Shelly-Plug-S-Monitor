FROM rust:1.88.0 AS backend-builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM node:16 AS frontend-builder
WORKDIR /usr/src/app
COPY frontend_vue .
RUN npm install
RUN npm run build

FROM debian:bookworm-slim
LABEL authors="spoomer"
WORKDIR /app
COPY --from=backend-builder /usr/src/app/target/release/shelly_web .
COPY --from=frontend-builder /usr/src/wwwroot ./wwwroot
EXPOSE 80
ENTRYPOINT ["./shelly_web"]