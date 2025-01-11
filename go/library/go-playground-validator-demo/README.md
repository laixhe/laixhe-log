### 参数校验库

> 安装

```
go get github.com/go-playground/validator/v10
```
> 导入
```
import "github.com/go-playground/validator/v10"
```

```
required     非空
numeric      数字类型
dive         数组

startswith   以参数子串为前缀，例如startswith=hi
endswith     以参数子串为后缀，例如endswith=bye

ne      不等于参数值，例如ne=5
gt      大于参数值，例如gt=5
gte     大于等于参数值，例如gte=5
lt      小于参数值，例如lt=5
lte     小于等于参数值，例如lte=5

len     等于参数值，例如len=10
max     小于等于参数值，例如max=10
min     大于等于参数值，例如min=10

oneof   只能是列举出的值其中一个，这些值必须是数值或字符串，以空格分隔
        例如oneof=male female
```
