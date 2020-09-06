const autocannon = require('autocannon')
const utils = require('./utils')
const graphql = require("./graphql")

let abort=false

function saveResults(err, result){
  if (abort===true) {
    console.log("Load test cancelled...")
    return
  }
  utils.saveToLowdb(err,result)
}

const loadTest = autocannon({
  ...utils.settings,
  title:"actix-gql-todo",
  url:"http://localhost:8080",
  requests:[{
      method:'POST',
      path:'/v1/graphql',
      headers:{
        'content-type':'application/json',
        'autohorization':'Bearer FAKE_JWT_KEY'
      },
      body:JSON.stringify(graphql.getTodoList),
    },{
      method:'POST',
      path:'/v1/graphql',
      headers:{
        'content-type':'application/json',
        'autohorization':'Bearer FAKE_JWT_KEY'
      },
      body:JSON.stringify(graphql.addTodoList),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          if (resp['data'] && resp['data']['createTodoList']){
            context['list_id'] = resp['data']['createTodoList']['id']
          }else{
            console.log("failed to extract list_id")
          }
        }
      }
    },{
      method:'POST',
      path:'/v1/graphql',
      setupRequest:(req, context)=>({
        ...req,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify(graphql.updateTodoList(
          context['list_id'],"Autocannon title update")
        )
      })
    },{
      method: 'POST',
      path:'/v1/graphql',
      setupRequest:(req, context)=>({
        ...req,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify(graphql.addTodoItem(
          context['list_id'],
          "Item title",
          false
        )),
      }),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          // console.log("todo_id", resp['payload']['id'])
          if (resp['data'] && resp['data']['createTodoItem']){
            context['todo_id'] = resp['data']['createTodoItem']['id']
          } else {
            console.log("failed to extract todo_id")
          }
        }
      }
    },{
      method: 'POST',
      path:'/v1/graphql',
      setupRequest:(req, context)=>({
        ...req,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify(graphql.getTodoItemForList(
          context['list_id']
        ))
      })
    },{
      method: 'POST',
      path:'/v1/graphql',
      setupRequest:(req, context)=>{
        let id=1
        if (context['todo_id']){
          id=context['todo_id']
        }
        return {
          ...req,
          headers:{
            'content-type':'application/json',
            'autohorization':'Bearer FAKE_JWT_KEY'
          },
          body:JSON.stringify(graphql.deleteTodoItem(id))
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