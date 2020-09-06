module.exports={
  // Test data for post and put
  todoList:{
    title:"New todo list"
  },
  todoItem:(lid)=>({
    list_id:lid,
    title:"New item title from autocanon",
    checked: false
  }),

}