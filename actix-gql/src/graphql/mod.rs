use crate::db::Pool;
use juniper::RootNode;

mod model;
mod mutation;
mod query;

use mutation::MutationRoot;
use query::QueryRoot;

// GraphQL context used
// to pass db pool
pub struct Context {
  pub db: Pool,
}
impl juniper::Context for Context {}

pub use model::{NewTodoItem, TodoItem, TodoList};

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(QueryRoot {}, MutationRoot {})
}
