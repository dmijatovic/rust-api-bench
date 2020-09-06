# Actix todo graphQL api

This example is attempt to implement graphQL with actix-web v3. This is new version of actix still in beta.

For graphQL implementation most popular solution seem to be juniper.

Note! The performance of this api is measured using the same tool (autocannon) as I did in my [todo-api-bench](https://github.com/dmijatovic/todo-api-bench) project. `This approach is slower that any approach used in todo-api-bench project`. The load test (30 sec.) with this api produces about 30k successfull request, while actix-web without graphQL produces about 170k successfull request on the same machine. This is quite significant difference in the performance.

## Usage

This test todo api with actix and juniper (GraphQL) can use runned using docker-compose

```bash
# start api and postgres
docker-compose up -d
# stop and remove containers
docker-compose down --volumes
```

### Environment variables

Api uses following environment variables. The variables are passed using docker-compose. The default values are displayed here (and are embeded in the source code)

```bash
# actix-web
SERVER_HOST=localhost
SERVER_PORT=8080
SERVER_WORKERS=2
# postgres
PG_HOST=pgdb
PG_PORT=5432
PG_USER=postgres
PG_PASSWORD=changeme
PG_DBNAME=todo_db
PG_POOL_SIZE=30
PG_TIMEOUT=30
```

## Implementation challenges

I managed to setup basic server, which was simple to do based on documentation, googling on internet and watching some [youtube videos](https://www.youtube.com/watch?v=9q4GcWbAIEM&list=PLRiZb4DNOVQduDWGbqZR3bB4O4D9UDmIQ&index=9&t=0s). However after using deadpool postgres connection pool to setup async connection there were problems. To use async/await with juniper at this point in time special git version (and branch) should be included in cargo.toml file

```yml
[dependencies]
juniper = { git = "https://github.com/graphql-rust/juniper", branch = "async-await", features = ["async"] }
```

Unfortunately after including this version of juniper into a project the build crashed due to version coalision of future crate used by juniper and actix-web v3. As both libraries are in beta/instable state at the moment there seem to be not mutch I can do about it. I choosed to go back to sync juniper and keep v3 actix-web in use. To solve async calls to postgres from juniper I used tokio runtime to await for future to resolve.
