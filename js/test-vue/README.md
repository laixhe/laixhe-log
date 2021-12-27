# test-vue

## Project setup
```
npm install
```

### Compiles and hot-reloads for development
```
npm run serve
```

### Compiles and minifies for production
```
npm run build
```

### Lints and fixes files
```
npm run lint
```

#### 创建项目
```
创建 test-vue 项目：vue create test-vue
这里选择第三种：Manually select features

# 选择配置
(*) Choose Vue version  ：选择 Vue 版本
(*)Babel                ：支持使用Babel编译器
(*)TypeScript           ：支持使用 TypeScript 书写源码
( )Progressive Web App (PWA) Support ：支持PWA
(*) Router              ：支持 vue-router
(*)Vuex                 ：支持 vuex
(*)CSS Pre-processors   ：支持 CSS 预处理器
(*)Linter / Formatter   ：支持代码风格检查和格式化
( )Unit Testing         ：支持单元测试
( )E2E Testing          ： 支持 E2E 测试

# 是否使用 class 风格的组件语法
Use class-style component syntax? (y/N) y

# 是否使用 Babel 与 TypeScript 一起用于自动检测的填充
Use Babel alongside TypeScript (required for modern mode, auto-detected polyfills, transpiling JSX)? (Y/n) Y

# 是否使用 history 路由模式
Use history mode for router? (Requires proper server setup for index fallback in production) (Y/n) Y

# 选择CSS预处理器
node-sass：是用 node(调用 cpp 编写的 libsass)来编译 sass，是自动编译实时的
dart-sass：是用 drat VM 来编译 sass，需要保存后才会生效
这里选择：Sass/SCSS（with node-sass）

# 选择 eslint 配置
ESLint with error prevention only：只进行报错提醒；
ESLint + Airbnb config：Airbnb配置，不严谨模式；
ESLint + Standard config：标准配置，正常模式；
ESLint + Prettier：严格模式；
TSlint：typescript格式验证工具
这里选择：ESLint with error prevention only

# 选择什么时候执行 eslint 校验
Lint on save：保存时检查
Lint and fix on commit：提交时检查
这里选择：Lint on save

# 选择配置文件存放的位置
In dedicated config files：在专用的配置文件中单独存放
In package.json：存放在 package.json 中
这里选择：In dedicated config files

# 是否保存之前的配置项
Save this as a preset for future projects? (y/N) N
```


#### setup 函数
```
setup 函数是 Vue3 中新增的函数，它是我们在编写组件时，使用 Composition API 的入口。
同时它也是 Vue3 中新增的一个生命周期函数，会在 beforeCreate 之前调用。
因为此时组件的 data 和 methods 还没有初始化，因此在 setup 中是不能使用 this 的。
所以 Vue 为了避免我们错误的使用，它直接将 setup 函数中的 this 修改成了undefined 并且，
我们只能同步使用setup函数，不能用async将其设为异步。

setup 函数接收两个参数 props和 context， 语法为：setup(props,context){}


# props

props 里面包含父组件传递给子组件的所有数据。在子组件中使用 props 进行接收。

props 是响应式的， 当传入新的 props 时，会及时被更新。
由于是响应式的， 所以不可以使用 ES6 解构，解构会消除它的响应式。


# 对比

  Vue2.x	      Vue3.x
beforeCreate	  setup
created	          setup
beforeMount	      onBeforeMount
mounted	          onMounted
beforeUpdate	  onBeforeUpdate
updated	          onUpdated
beforeDestroy	  onBeforeUnmount
destroyed	      onUnmounted
activated	      onActivated
deactivated	      onDeactivated`
errorCaptured	  onErrorCaptured
- -	              onRenderTracked
- -	              onRenderTriggered


# 初始化加载顺序

setup => beforeCreate => created => onBeforeMount => onMounted


# 创建一个响应式数据

ref 与 reactive
ref：任意类型（建议基本类型）数据的响应式引用（设置、获取值时需要加.value）
     本质是拷贝，修改数据是不会影响到原始数据
reactive：只能是复杂类型数据的响应式引用


# 创建一个响应式数据

toRef 与 toRefs
toRef：用来给抽离响应式对象中的某一个属性，并把该属性包裹成 ref 对象，使其和原对象产生链接。
toRef 的本质是引用，修改响应式数据会影响原始数据。
toRefs：用来把响应式对象转换成普通对象，把对象中的每一个属性，包裹成 ref 对象。
toRefs 就是 toRef 的升级版，只是toRefs 是把响应式对象进行转换，其余的特性和 toRef 无二

readonly 只读属性
computed 计算属性
watch 与 watchEffect 监听属性
```

#### Vuex 是一个专为 Vue 开发的应用程序的状态管理模式
> 状态存储是响应式
