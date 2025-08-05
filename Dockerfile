###### WASM Builder
FROM rust:1.84 AS wasm_builder

RUN cargo install wasm-pack

WORKDIR /src

COPY algorithm /src

RUN wasm-pack build

###### Database Builder
FROM rust:1.84 AS db_builder

WORKDIR /src

COPY database /src

RUN apt-get update && apt-get install musl-tools -y
RUN rustup target add x86_64-unknown-linux-musl
RUN SQLX_OFFLINE=true cargo build --release --target x86_64-unknown-linux-musl

###### Final
FROM node:24-slim

RUN npm install -g pnpm

WORKDIR /app

COPY web/pnpm-lock.yaml web/package.json ./

# RUN pnpm install
RUN pnpm install; cd node_modules/better-sqlite3; pnpm run install

COPY web /app
RUN mkdir /app/src/lib/pkg; mkdir /database; apt-get update; apt-get install -y cron

COPY --from=wasm_builder /src/pkg /app/src/lib/pkg
COPY --from=db_builder /src/target/x86_64-unknown-linux-musl/release/database /database/database
COPY docker/run_app /run_app
COPY database/config.toml /database/
COPY docker/crontab /etc/cron.d/database-refresh
RUN chmod 0644 /etc/cron.d/database-refresh; crontab /etc/cron.d/database-refresh
RUN sed -i 's;"data.db";"/app/data/data.db";' /database/config.toml

RUN pnpm run build

ENV ORIGIN=http://localhost:3000

EXPOSE 3000

VOLUME ["/app/data/"]

CMD ["/run_app"]
