import { createStore } from 'vuex'

// 包含模块
/**
State：定义了应用状态的数据结构，可以在这里设置默认的初始状态。
Getter：允许组件从 store 中获取数据，mapGetters 辅助函数仅仅是将 store 中的 getter 映射到局部计算属性。
Mutation：是唯一更改 store 中状态的方法，且必须是同步函数。
Action：用于提交 mutation，而不是直接变更状态，可以包含任意异步操作。
Module：可以将 store 分割成模块（module）。每个模块拥有自己的 state、mutation、action、getter、甚至是嵌套子模块
 */
export default createStore({
  // 存储
  state: {
    // this.$store.state.count;
    count:5
  },
  // 获取
  getters:{
    // this.$store.getters.getCount;
    getCount(state): number{
      return state.count * 3
    }
  },
  // 修改，同步
  mutations: {
    // this.$store.commit("setCount", value);
    setCount(state, value: number): void{
        state.count += value;
    }
  },
  // 通过 mutations 修改，可异步
  actions: {
    // this.$store.dispatch('actSetCount', value)
    actSetCount(store, value: number): void{
      store.commit('setCount', value);
    }
  },
  modules: {
  }
})
