<template>
	<div>{{name}}:{{age}}</div>
</template>

<script>
import { toRefs } from 'vue'

export default {

	props: {
		name: String,
		age: Number
	},

	setup (props, context) {
        //let { attrs, slots, emit } = context;
        // setup 函数接收两个参数 props 和 context
        // context 里面包含
        //         attrs：获取组件上的属性
        //         slots：获取 slot 插槽的节点
        //         emit ：emit 方法（子组件向父组件传递数据）

        // 获取父组件传递给子组件的数据
		console.log(props.name);
		console.log(props.age);
		
		// 直接解构
		let { name, age } = toRefs(props);
		console.log(name.value, age.value);

        return {}
	},

	created(){
		console.log("state.count=" + this.$store.state.count);
	},

}

</script>
