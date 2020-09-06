use juniper::{GraphQLInputObject, GraphQLObject};

// if you need to pass this struct as parameter
// in GraphQL mutations it needs to derive GraphQLInputObject
#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "Todo List. Represents group of todo items with same goal.")]
pub struct TodoList {
  pub id: i32,
  pub title: String,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New todo list to be created.")]
pub struct NewTodoList {
  pub title: String,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "Todo list to be updated.")]
pub struct UpdateTodoList {
  pub id: i32,
  pub title: String,
}

#[derive(GraphQLObject, Debug, Clone)]
#[graphql(description = "Todo Item. Represents single todo item belonging to specific todo list.")]
pub struct TodoItem {
  pub id: i32,
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}

#[derive(GraphQLInputObject, Debug)]
#[graphql(description = "New todo item to be created in specific todo list.")]
pub struct NewTodoItem {
  pub list_id: i32,
  pub title: String,
  pub checked: bool,
}
