const autocannon = require('autocannon')
const utils = require('./utils')

let abort=false

let noId={
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
    }
  })
  console.log("IdNotRetuned: ", (noId.list + noId.item))
}

const loadTest = autocannon({
  ...utils.settings,
  title:"actix-v2-dp",
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
      body:JSON.stringify({
        title:"New todo list"
      }),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          if (resp['id']){
            context['list_id'] = resp['id']
          }else{
            noId.list+=1
          }
        }
      }
    },{
      method:'PUT',
      path:'/todolist',
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
      path: "/todo",
      setupRequest:(req, context)=>({
        ...req,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify({
          "list_id": context['list_id'],
          "title":"Todo item",
          "checked": false
        })
      }),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          if (resp['id']){
            context['todo_id'] = resp['id']
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