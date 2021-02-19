<template>
  <div>
    <!-- Vue2 的方式实现 -->
    <h2>{{countV2}}</h2>
    <button @click="updateCountV2">更新数据V2</button>
  </div>

  <div>
    <!-- Vue3 的方式实现 -->
    <h2>{{countV3}}</h2>
    <button @click="updateCountV3">更新数据V3</button>
  </div>
  
</template>

<script lang="ts">
// 导入需要的组件
import { defineComponent, ref } from 'vue';

// 自定义组件并导出
export default defineComponent({
  // 当前组件的名字
  name: 'App',
  
  // 在创建组件之前执行，是组合API的入口函数
  // 返回对象在模板中可以直接使用
  // Vue3 的方式实现
  setup(){
    // 定义一个基本类型的响应式数据
    const countV3 = ref(0);
    function updateCountV3(){
      countV3.value++;
    }

    return {
      countV3,
      updateCountV3
    }
  },

  // Vue2 的方式实现
  // 数据
  data(){
    return {
      countV2: 0
    }
  },
  // 方法
  methods:{
    updateCountV2(){
      this.countV2++;
    }
  },

  // 注册其它组件
  components: {
  }
});
</script>
