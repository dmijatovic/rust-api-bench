const autocannon = require('autocannon')
const utils = require('./utils')
const todos = require("./todos")


let abort=false
const noId={
  list:0,
  item:0
}
const created={
  list:0,
  item:0
}

function saveResults(err, result){
  if (abort===true) {
    console.log("Load test cancelled...")
    return
  }
  utils.saveToLowdb(err,{
    ...result,
    IdNotRetuned:{
      ...noId
    },
    Created:{
      ...created
    }
  })
}

const loadTest = autocannon({
  ...utils.settings,
  title:"hyper-dp",
  requests:[{
      method:'GET',
      path:'/',
    },{
      method:'GET',
      path:'/todolist',
    },{
      method:'POST',
      path:'/todolist',
      headers:{
        'content-type':'application/json',
        'autohorization':'Bearer FAKE_JWT_KEY'
      },
      body:JSON.stringify(todos.todoList),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          if (resp && resp.length===1){
            context['list_id'] = resp[0]['id']
            created.list+=1
          } else {
            noId.list+=1
          }
        }
      }
    },{
      method:'PUT',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todolist`,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify({
          id: context['list_id'],
          title:"Autocannon title update"
        })
      })
    },{
      method: 'POST',
      setupRequest:(req, context)=>({
        ...req,
        path:"/todo",
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify(todos.todoItem(context['list_id']))
      }),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          if (resp && resp.length===1){
            context['todo_id'] = resp[0]['id']
            created.item+=1
          }else{
            noId.item+=1
          }
        }
      }
    },{
      method: 'GET',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todolist/${context['list_id']}/items`,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        }
      })
    },{
      method:"DELETE",
      setupRequest:(req, context)=>{
        let id=1
        if (context['todo_id']){
          id=context['todo_id']
        }
        return {
          ...req,
          path:`/todo/${id}`,
          headers:{
            'content-type':'application/json',
            'autohorization':'Bearer FAKE_JWT_KEY'
          }
        }
      }
    }
  ]
},saveResults)

process.once('SIGINT',()=>{
  abort = true
  loadTest.stop()
})

autocannon.track(loadTest)