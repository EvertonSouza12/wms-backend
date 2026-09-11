# Etapa 1: Compilação do binário
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Etapa 2: Execução em ambiente enxuto
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/target/release/wms-backend /usr/local/bin/
EXPOSE 3000
CMD ["wms-backend"]