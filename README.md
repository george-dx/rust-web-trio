# RUST WEB TRIO: AXUM, AXIOS, ROCKET

This document explains how to use the project as a learning material for AXUM vs AXIOS vs ROCKET frameworks

<!-- TOC -->
* [RUST WEB TRIO: AXUM, AXIOS, ROCKET](#rust-web-trio-axum-axios-rocket)
  * [Prerequisites](#prerequisites)
    * [PostgreSQL server setup](#postgresql-server-setup)
    * [Environment variable setup](#environment-variable-setup)
  * [Testing the binaries](#testing-the-binaries-)
    * [Axum](#axum-)
    * [Axios](#axios-)
    * [Rocket](#rocket)
  * [Conclusion](#conclusion-)
<!-- TOC -->

## Prerequisites

### PostgreSQL server setup
If you’re using RustRover 2024, you can configure a local PostgreSQL instance easily:

1. Open the Database tool window.
2. Click __+__ -> __Data Source__ -> __PostgreSQL__.
3. Enter your __username__ and __password__.
   * Everything else can remain as default: database name `postgres`, host `localhost`, port `5432`.
4. Test the connection to confirm it works.

### Environment variable setup
Set your database connection URL so the application can connect to PostgreSQL:

```shell
export DATABASE_URL=postgres://{username}:{password}@localhost/postgres
```

Replace `{username}` and `{password}` with your PostgreSQL credentials. This environment variable is required by all examples (Axum, Actix, Rocket) to establish a database connection.

## Testing the binaries 

```shell
cargo run --bin axum_app
```

### Axum 
* ``GET /health``
```bash
curl -i http://127.0.0.1:3000/health
```

* ``POST /users``
```bash
curl -i -X POST http://127.0.0.1:3000/users \
-H "Content-Type: application/json" \
-d '{"id":1,"name":"Joe"}'
```

* ``GET /users``
```bash
curl -i http://127.0.0.1:3000/users/1
```

### Axios 
Same as for __Axum__ but on port `3001`

### Rocket
Same as for __Axum__ but on port `8000`

## Conclusion 

From my own experience and research, I’d summarize the three main Rust web frameworks as follows:
- ``Axum``'s close tie with Tokio gives confidence for async ecosystem. Use when you are building an async-first microservice or API in Rust.
- ``Actix Web``'s maturity gives plugin richness and is in between Axum and Rocket from ergonomics perspective. Use when you need maximum performance, a very mature ecosystem with many ready-out-of-box middleware/plugins (sessions, websockets, streaming).
- ``Rocket`` is much more pleasant for day-to-day development and prototyping. Use when developer productivity, ergonomics, and fast iteration matter more than squeezing every last microsecond, especially for internal tools, prototypes, smaller services.
