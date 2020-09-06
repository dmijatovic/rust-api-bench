const autocannon = require('autocannon')
const utils = require('./utils')

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
  title:"actix-v2-dp",
  requests:[{
      method:'GET',
      path:'/',
    },{
      method:'POST',
      path:'/todos',
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
          context['list_id'] = resp['id']
        }
      }
    },{
      method:'PUT',
      path:'/todos',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todos`,
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
        path:`/todos/${context['list_id']}/items`,
        headers:{
          'content-type':'application/json',
          'autohorization':'Bearer FAKE_JWT_KEY'
        },
        body:JSON.stringify({
          "title":"Todo item",
          "checked": false
        })
      }),
      onResponse:(status, body, context)=>{
        if (status === 200) {
          const resp = JSON.parse(body)
          context['todo_id'] = resp['id']
        }
      }
    },{
      method: 'GET',
      setupRequest:(req, context)=>({
        ...req,
        path:`/todos/${context['list_id']}/items`,
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
          path:`/todo/item/${id}`,
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