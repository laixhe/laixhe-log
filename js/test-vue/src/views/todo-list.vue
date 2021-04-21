<template>
  <div class="home">
    <!-- input输入list内容 -->
    <div>
       <input
    @keyup.enter="addTask"
      class="input"
      type="text"
      v-model="inputValue"
      placeholder="请输入" />
    </div>
    <!-- todoList内容展示和删除 -->
     <ul class="ul">
      <li class="item" v-for="(item, index) in taskList" :key="index">
        <p
        @click="updateStatus(index, !item.isfinished)"
        class="content"
        :class="item.isfinished ? 'active' : ''"
        >{{item.lable}}</p>
        <div class="item-delete" @click="deleteTask(index)">X</div>
      </li>
      <li v-if="taskList.length === 0" class="item-none">暂无数据</li>
    </ul>
  </div>
</template>
<script lang="ts">
 
import { ref, computed } from 'vue';
import { useStore } from "vuex";
export default {
  
  setup() {
    const store = useStore()
    const taskList = computed(() => store.state.taskList);
    const inputValue = ref('');
    const addTask = () => {
      store.commit('createTask', {
        lable: inputValue.value,
        isfinished: false
      })

      inputValue.value = ''
    }

    const updateStatus = (index, status) => {
      store.commit('updateStatus', {
        index,
        status
      })
    }

    const deleteTask = (index) => {
      store.commit('deleteTask', index)
    }

    return {
      inputValue,
      taskList,
      addTask,
      updateStatus,
      deleteTask
    };
  }
}
</script>

<style scoped lang='scss'>
* {
  box-sizing: border-box;
  margin: 0;
  padding: 0;
}
ul,
li {
  list-style: none;

  text-align: left;
}
.home {
  max-width: 400px;
  margin: 0 auto;
  .input {
    width: 100%;
    height: 40px;
    border-radius: 5px;
    outline-style: none;
    border: 2px solid #999;
    padding: 5px 10px;
  }
  .ul {
    margin-top: 10px;
  }

  .item {
    height: 40px;
    line-height: 40px;
    padding-bottom: 5px;
    border-bottom: 1px solid #dcdfe6;
    color: #333333;
  }
  .item-none {
    height: 40px;
    line-height: 40px;
    padding-bottom: 5px;
    color: #333333;
    text-align: center;
  }
  .content {
    float: left;
    height: 40px;
    line-height: 40px;
    cursor: pointer;
  }
  p.active {
    text-decoration:line-through; 
    color: #999999;
  }
  .item-delete {
    float: right;
    width: 25px;
    text-align: center;
    cursor: pointer;
  }
}
</style>
