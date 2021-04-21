import { createStore } from 'vuex'

interface State {
  userName: string
  taskList: any[]
  
}

export default createStore({
  state: {
    
      userName: "王大合",
      taskList: []
      
  },
  mutations:{
    
    createTask (state:any, newTask:string) {
        state.taskList.push(newTask)
      },
      deleteTask (state:any, index:number) {
        state.taskList.splice(index, 1)
      },
      updateStatus (state:any, payload:any) {
        const { index, status } = payload
  
        state.taskList[index].isfinished = status
      }
  }
});