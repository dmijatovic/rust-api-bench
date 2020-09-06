use juniper::FieldResult;
use tokio;

use super::model::{TodoItem, TodoList};
use super::Context;
use crate::db::{todo_item, todo_list};

pub struct QueryRoot;

#[juniper::object(
  // Here we specify the context type for this object.
  Context = Context,
)]
impl QueryRoot {
  #[graphql(description = "List of all todo lists")]
  fn todo_lists(ctx: &Context) -> FieldResult<Vec<TodoList>> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_list::get_todos(&ctx.db)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // println!("todo_lists: {:?}", todo);
    // return todo
    Ok(todo)
  }
  #[graphql(
    description = "Single todo list item in an array. If item is not found empty array is returned."
  )]
  fn todo_list_item(id: i32, ctx: &Context) -> FieldResult<Vec<TodoList>> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_list::get_todo_list(&ctx.db, id)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    Ok(todo)
  }
  #[graphql(
    description = "Single todo item return in array. If item is not found empty array is returned."
  )]
  fn todo_item(lid: i32, id: i32, ctx: &Context) -> FieldResult<Vec<TodoItem>> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_item::get_todo_item(&ctx.db, id)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    Ok(todo)
  }
  #[graphql(description = "All todo items from specified todo list.")]
  fn todo_items_for_list(lid: i32, ctx: &Context) -> FieldResult<Vec<TodoItem>> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_item::get_todo_items(&ctx.db, lid)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    Ok(todo)
  }
}
