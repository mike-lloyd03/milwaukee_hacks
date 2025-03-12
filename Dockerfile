FROM rust:1.84 AS rust_builder

RUN cargo install wasm-pack

WORKDIR /src

COPY algorithm /src

RUN wasm-pack build

FROM node:23-alpine

RUN npm install -g pnpm

WORKDIR /app

COPY web/pnpm-lock.yaml web/package.json ./

RUN pnpm install

COPY web /app
COPY --from=rust_builder /src/pkg /app/src/lib/pkg

RUN pnpm run build

ENV ORIGIN=http://localhost:3000

EXPOSE 3000

CMD ["node", "build"]
