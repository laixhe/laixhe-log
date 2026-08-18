# TypeScript 模块导出

TypeScript 支持两种模块体系：ES Module（`import` / `export`，现代主流）和 CommonJS（`require` / `module.exports`，Node.js 传统）。推荐优先使用 ES Module。

## ES Module（推荐）

```typescript
// 具名导出
export const name = 'laixhe';
export function greet() {
    return 'hello';
}

// 默认导出
export default class User {
}
```

```typescript
// 导入
import User, { name, greet } from './module';
```

## CommonJS：exports 和 module.exports 的区别

在 Node.js 的 CommonJS 模块中，`require()` 真正返回的是 `module.exports`。`exports` 只是 `module.exports` 的一个引用，初始时二者指向同一个空对象。

### 方式一：挂属性（exports 和 module.exports 等价）

```typescript
exports.funcUser = function () {};
// 等价于
module.exports.funcUser = function () {};
```

此时 `exports` 和 `module.exports` 指向同一个对象，两者挂属性的效果相同。

### 方式二：整体替换（必须用 module.exports）

```typescript
class User {
}
module.exports = User;
```

如果直接给 `module.exports` 重新赋值（导出一个类/函数），`require()` 拿到的是这个新值。

> 注意：此时不能用 `exports = User`。因为那只是把 `exports` 变量指向了新对象，`module.exports` 并没有变化，导出会失效。

### 小结

| 写法 | 效果 |
| --- | --- |
| `exports.xxx = ...` | 往导出对象上挂属性 |
| `module.exports = ...` | 整体替换导出值（导出类 / 函数 / 对象） |

> 记不住就统一用 `module.exports`，最不容易出错。
