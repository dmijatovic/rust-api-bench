module.exports={
  getTodoList:{
    "query":"query{\n  todoLists{\n    id,\n    title\n  }\n}",
    "variables":null
  },
  addTodoList:{
    "query":"mutation{  \n  createTodoList(newList:{title:\"This is my list title\"}){\n    id,\n    title\n  }\n}",
    "variables":null
  },
  updateTodoList:(id=1,title="default title")=>({
    "query":`mutation{  \n  updateTodoList(item:{id:${id},title:\"${title}\"}){\n    id,\n    title\n  }\n}`,
    "variables":null
  }),
  addTodoItem:(list_id=1,title="",checked=false)=>({
    "query":`mutation{  \n  createTodoItem(newItem:{listId:${list_id},title:\"${title}\", checked:${checked}}){\n    id,\n    listId,\n    title,\n    checked\n  }\n}`,
    "variables":null
  }),
  getTodoItemForList:(list_id=1)=>({
    "query":`query{\n  todoItemsForList(lid:${list_id}){\n    id,\n    listId,\n    title,\n    checked\n  }\n}`,
    "variables":null
  }),
  deleteTodoItem:(id)=>({
    "query":`mutation{\n  deleteTodoItem(id:${id}){\n    id,\n    listId,\n    title,\n    checked\n  }\n}`,
    "variables":null
  })
}