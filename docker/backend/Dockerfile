FROM rust:1.70.0-slim-buster

WORKDIR /app

RUN apt-get install -y tzdata

ENV DEBIAN_FRONTEND=noninteractive \
    LANGUAGE=C.UTF-8 \
    ANG=C.UTF-8 \
    LC_ALL=C.UTF-8 \
    LC_CTYPE=C.UTF-8 \
    LC_MESSAGES=C.UTF-8 \
    TZ=America/Sao_Paulo

RUN apt-get update && \ 
    # Timezone
    apt-get install -y tzdata && \ 
    ln -snf /usr/share/zoneinfo/$TZ /etc/localtime && echo $TZ > /etc/timezone && \
    apt-get update && \
    apt install -y --no-install-recommends \
    gcc make build-essential wget curl && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

RUN cargo install --path /app/BookBorrow
CMD ["BookBorrow"]
# CMD ["tail", "-f", "/dev/null"]