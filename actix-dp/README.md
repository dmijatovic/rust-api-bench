# Actix web with deadpool and tokio-postgres

This project is testbed for actix-web todo app with deadpool and tokio-postgres.

**This version of actix-api is significantly slowe than v2. Further investigation is needed to discover why?**.

## Remarks

Initial version of this api was quite slow (<50k request on load test). There are two improvements I made based on api version I created earlier which had quite high performance.

- Using actix-web json response instead using body

```rs
// SLOW SOLUTION (ca. 50k)
match todo_item::get_todo_items(&pool, *lid).await {
  //if request is ok return json data
  Ok(res) => HttpResponse::Ok()
    .content_type("application/json")
    .body(serde_json::to_string(&res).unwrap()),
  //if error create server serror json response
  Err(e) => {
    // error macro will report module::fn automatically
    error!("{:?}", e);
    let res = response::server_error(format!("{:?}", e));
    HttpResponse::InternalServerError()
      .content_type("application/json")
      .body(serde_json::to_string(&res).unwrap())
  }
}
// FAST SOLUTION (ca. 200k)!!!
match todo_item::get_todo_items(&pool, *lid).await {
  //if request is ok return json data
  Ok(res) => HttpResponse::Ok().json(res),
  //if error create server serror json response
  Err(e) => {
    // error macro will report module::fn automatically
    error!("{:?}", e);
    let res = response::server_error(format!("{:?}", e));
    HttpResponse::InternalServerError().json(res)
  }
}
```

- Using tokio_pg_mapper to convert sql row into vector of todo structs

```rs

// SLOW SOLUTION (ca. 50k)
// map return from tokio:postreges into stuct
impl From<&Row> for TodoList {
  fn from(row: &Row) -> Self {
    Self {
      id: row.get("id"),
      title: row.get("title"),
    }
  }
}
let rows = cnn
    .query(&sql, args)
    .await?
    .iter()
    .map(|row: &Row| TodoList::from(row))
    .collect::<Vec<TodoList>>();

// FAST SOLUTION (ca. 200k)!!!
// execute query and collect results
let rows = cnn
  .query(&sql, args)
  .await?
  .iter()
  //this might be performance efficient solution
  .map(|row| TodoItem::from_row_ref(row).unwrap())
  .collect::<Vec<TodoItem>>();

```
