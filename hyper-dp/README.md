# Hyper todo api with deadpool and tokio-postgres

This api is testbed for hyper http server using todo-dp (local) library.

todo_dp library is simple todo CRUD handler for Postgres using deadpool and tokio. Database operations are async.

## Remarks

Hyper seem to be quite low level. It lacks variable routes. Extracting request body requires converting stream request into bytes and use of serde deserializer.

```rs
// extract body from request stream
let full_body = hyper::body::to_bytes(req.into_body()).await.unwrap();
// deserialize bytes into struct
let list: NewTodoList = serde_json::from_slice(&full_body).unwrap();
// optional deserialize from string
// let list: NewTodoList = serde_json::from_str(&list_str).unwrap();

// OR using aggregate
let full_body = hyper::body::aggregate(req.into_body()).await.unwrap();
let item: NewTodoItem = serde_json::from_reader(full_body.reader()).unwrap();
```
