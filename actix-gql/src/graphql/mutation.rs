use super::model::{NewTodoItem, NewTodoList, TodoItem, TodoList, UpdateTodoList};
use super::Context;
use crate::db::{todo_item, todo_list};
use juniper::FieldResult;
use tokio;

pub struct MutationRoot;

#[juniper::object(Context = Context)]
impl MutationRoot {
  /// Provide list title. It returns array with created list with unique id.
  fn create_todo_list(new_list: NewTodoList, ctx: &Context) -> FieldResult<TodoList> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_list::add_todo_list(&ctx.db, &new_list.title)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo list item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(String::from(
        "Failed to create todo list. Check values provided in the query.",
      ))?
    }
  }
  /// Provide existing id and the title. It returns updated list item.
  fn update_todo_list(item: UpdateTodoList, ctx: &Context) -> FieldResult<TodoList> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_list::update_todo_list(&ctx.db, item.id, &item.title)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo list item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(format!(
        "Failed to update todo list with id {}. Check values provided in the query.",
        item.id
      ))?
    }
  }
  /// Provide list id. It returns deleted item.
  fn delete_todo_list(id: i32, ctx: &Context) -> FieldResult<TodoList> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_list::delete_todo_list(&ctx.db, id)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo list item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(format!(
        "Failed to delete todo list with id {}. Check list id provided in the query.",
        id
      ))?
    }
  }
  /// Provide new item list_id, title and checked values. It returns created todo item with generated id.
  fn create_todo_item(new_item: NewTodoItem, ctx: &Context) -> FieldResult<TodoItem> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_item::add_todo_item(&ctx.db, &new_item)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo list item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(String::from(
        "Failed to create todo item. Check values provided in the query.",
      ))?
    }
  }
  /// Provide item id, list_id, title and checked values.
  fn update_todo_item(id: i32, item: NewTodoItem, ctx: &Context) -> FieldResult<TodoItem> {
    //compose item to update
    let item_to_update = TodoItem {
      id,
      list_id: item.list_id,
      checked: item.checked,
      title: item.title,
    };
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_item::update_todo_item(&ctx.db, &item_to_update)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(format!(
        "Failed to update todo item with id {}. Check list id provided in the query.",
        id
      ))?
    }
  }
  /// Provide item id. It returns delete item.
  fn delete_todo_item(id: i32, ctx: &Context) -> FieldResult<TodoItem> {
    // create new async runtime using tokio
    let mut rt = tokio::runtime::Runtime::new().unwrap();
    // run async fn get_todos using tokio runtime
    let todo = match rt.block_on(todo_item::delete_todo_item(&ctx.db, id)) {
      Ok(todo) => todo,
      Err(e) => Err(e)?,
    };
    // return todo list item (first item)
    if todo.len() == 1 {
      let data = todo[0].clone();
      Ok(data)
    } else {
      Err(format!(
        "Failed to delete todo item with id {}. Check id provided in the query.",
        id
      ))?
    }
  }
}
