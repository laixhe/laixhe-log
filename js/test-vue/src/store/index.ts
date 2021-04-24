import { createStore } from 'vuex'

export default createStore({
  state: {
    // 状态数据
    taskList: []
  },
  mutations:{
    // 自定义改变状态数据方法
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