#### 基本配置的说明

> 安装
```
go get go.uber.org/zap
```

> 导入
```
import "go.uber.org/zap"
```

Level：日志级别，跟其他语言是一样的。只不过它需要的类型是AtomicLevel。所以需要使用zap.NewAtomicLevelAt做下如下的转化。
```
zap.NewAtomicLevelAt(zap.DebugLevel)
zap.DebugLevel
zap.InfoLevel
zap.WarnLevel
zap.ErrorLevel
Development：bool       是否是开发环境。如果是开发模式，对DPanicLevel进行堆栈跟踪
DisableCaller：bool     禁止使用调用函数的文件名和行号来注释日志。默认进行注释日志
DisableStacktrace：bool 是否禁用堆栈跟踪捕获。默认对Warn级别以上和生产error级别以上的进行堆栈跟踪。
Encoding：              编码类型，目前两种json 和 console【按照空格隔开】,常用json
EncoderConfig：         生成格式的一些配置--TODO 后面我们详细看下EncoderConfig配置各个说明
OutputPaths：[]string   日志写入文件的地址
ErrorOutputPaths：[]string            将系统内的error记录到文件的地址
InitialFields：map[string]interface{} 加入一些初始的字段数据，比如项目名
如果想控制台输出，OutputPaths和ErrorOutputPaths不能配置为文件地址，而应该改为stdout。
```

##### EncoderConfig配置说明
```
MessageKey： 输入信息的key名
LevelKey：   输出日志级别的key名
TimeKey：    输出时间的key名
LineEnding： 每行的分隔符。基本zapcore.DefaultLineEnding 即"\n"
EncodeLevel：基本zapcore.LowercaseLevelEncoder。将日志级别字符串转化为小写
EncodeTime： 输出的时间格式
EncodeDuration：一般zapcore.SecondsDurationEncoder,执行消耗的时间转化成浮点型的秒
EncodeCaller：  一般zapcore.ShortCallerEncoder，以包/文件:行号 格式化调用堆栈
EncodeName：    可选值。
```
