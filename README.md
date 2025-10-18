# RUST WEB TRIO: AXUM, AXIOS, ROCKET

This document explains how to use the project as a learning material for AXUM vs AXIOS vs ROCKET frameworks

## Prerequisites

Set DATABASE_URL using the following command:

```shell
export DATABASE_URL=postgres://postgres:postgres@localhost/postgres
```

## Testing the binaries 

```shell
cargo run --bin axum_app
```

### Axum 
```bash
curl -i http://127.0.0.1:3000/health
```

```bash
curl -i -X POST http://127.0.0.1:3000/users \
-H "Content-Type: application/json" \
-d '{"id":1,"name":"Joe"}'
```

```bash
curl -i http://127.0.0.1:3000/users/1
```

### Axios 

### Rocket
