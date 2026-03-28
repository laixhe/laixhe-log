#### exports 和 module.exports 的区别

- `exports` 返回的是模块函数(可以直接调用)
- `module.exports` 返回的是模块对象本身，返回的是一个类(需要new对象之后才可以调用)

```
class User {
}
module.exports = User;
#
var funcUser = function(){}
exports.funcUser = funcUser;
```
