<template>

  <div>
    <h2>{{count}}</h2>
    <button @click="updateCount">更新数据</button>
  </div>

  <div>
    <h4>{{user.name}}</h4>
    <h4>{{user.age}}</h4>
    <h4>{{user.wife}}</h4>
    <button @click="updateUser">更新数据</button>
  </div>

  <div id="nav">
    <router-link to="/">Home</router-link> | 
    <router-link to="/todo-list">TodoList</router-link>
  </div>

  <router-view/>
  
</template>

<script lang="ts">
// 导入需要的组件
import { defineComponent, ref, reactive } from 'vue';

import axios from './utils/axios'

// 自定义组件并导出
export default defineComponent({
  // 当前组件的名字
  name: 'App',
  
  // 在创建组件之前执行，是组合API的入口函数
  // 返回对象中的属性或方法，在模板中可以直接使用
  setup(){

    // 定义一个基本类型的响应式数据
    const count = ref(0);
    function updateCount(){
      count.value++;
    }

    // 定义一个结构类型的响应式数据
    const user = reactive({
      name:'小明',
      age: 20,
      wife:{
        name: '小红',
        age: 18
      }
    });
    const updateUser = ()=>{
        user.name = '..小明..';
    }

    axios
        .get('/users/XPoet')
        .then((res) => {
          console.log('res: ', res)
        })
        .catch((err) => {
          console.log('err: ', err)
        })

    return {
      count,
      updateCount,
      user,
      updateUser
    }
  },
  // 注册其它组件
  components: {
  }

});
</script>

<style lang="scss">
  #app {
  font-family: Avenir, Helvetica, Arial, sans-serif;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  text-align: center;
  color: #2c3e50;
}

#nav {
  padding: 30px;

  a {
    font-weight: bold;
    color: #2c3e50;

    &.router-link-exact-active {
      color: #42b983;
    }
  }
}
</style>