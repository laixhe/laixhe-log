<template>
  <div class="about">
    <h1>{{name}}</h1>
  </div>
</template>

<script lang='ts'>
import { ref, watch } from 'vue';

export default {
  name: 'about',
  setup() {
    const name = ref('王大合出品');
    return {
      name,
     
    };
  }
};
</script>