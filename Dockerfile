# Etapa 1: Build
FROM rust:latest as builder

# Instala bibliotecas de sistema essenciais para compilação C/SSL/MySQL
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /usr/src/wms-backend

# Limita o Cargo a usar no máximo 2 processos em paralelo para não estourar a RAM
ENV CARGO_BUILD_JOBS=2

COPY Cargo.toml ./
COPY src ./src

# Desativa a checagem em tempo de compilação do SQLx (caso esteja usando)
ENV SQLX_OFFLINE=true

# Compila a aplicação
RUN cargo build --release

# Etapa 2: Runtime (Imagem leve de execução)
FROM debian:bookworm-slim

# Instala dependências de runtime necessárias
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    default-mysql-client \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copia o binário final
COPY --from=builder /usr/src/wms-backend/target/release/wms-backend /usr/local/bin/wms-backend

EXPOSE 8080

CMD ["wms-backend"]