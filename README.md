# Rust todo api bench

This repo is extension of [todo-api-bench](https://github.com/dmijatovic/todo-api-bench) project. Main conclusion from todo-api-bench project is that actix-web api is clearly the fastest solution I have tested. Based on this conclusion I decided to further explore Rust programming language and its ecosystem. In this project I explore various http server libraries and compare their performance in similair way as I did in todo-api-bench project. We still use Postgres todo database and autocannon for load tests.

## Test criteria

I want to test following features

- Serving static content from home page. The home page is static index.html with minimal CSS file and (same) favicon.png
- CRUD on todos stored in Postgres.
- Logging

### Tested api points

- /: root with static index.html, index.css and favion.png
- /todolist [GET]: get all todo list (LIMIT to 50)
- /todolist [POST]: add todo list
- /todolist [PUT]: update specific todo list title
- /todo [POST]: add todo item to specific todo list
- /todolist/{id}/items [GET]: get items from specific todolist
- /todo/{id} [DELETE]: remove todo item

### Postgres setup

Postgres setup should be identical (as far as libraries allow) to following. I use env variables for this purpose.

```bash
PG_HOST=pgdb
PG_PORT=5432
PG_USER=postgres
PG_PASSWORD=changeme
PG_DBNAME=todo_db
PG_POOL_SIZE=30
PG_TIMEOUT=30
```

The most important vars are pool size (30 connections) and connection timeout (30 sec.).
