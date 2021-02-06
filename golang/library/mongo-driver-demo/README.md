### MongoDB 官方驱动

> 安装
```
go get go.mongodb.org/mongo-driver
```

> 导入
```
import "go.mongodb.org/mongo-driver/mongo"
```

##### 查找 name 字段与 张三 或 李四 匹配的文档
```
bson.D{{
   "name",
   bson.D{{
      "$in",
      bson.A{"张三", "李四"},
   }},
}}
```

##### 表示对 age 的值减一
```
bson.M{"$inc": bson.M{ "age": -1, }}
```

##### 使用 $push 可以对该字段增加一个元素,表示对 as 字段的元素数组增加 zz 元素
##### 使用 $pull 可以对该字段删除一个元素,表示对 as 字段的元素数组增加 zz 元素
```
bson.M{"$push": bson.M{ "as": "zz"}}
bson.M{"$pull": bson.M{ "as": "zz"}}
```

```
// 事务执行
err = db.Client().UseSession(context.Background(), func(sessionContext mongo.SessionContext) error {
    err := sessionContext.StartTransaction()
    if err != nil {
        return err
    }

    // 在事务内写一条记录
    _, err = table.InsertOne(sessionContext, bson.M{"name": "test", "age": 12})
    if err != nil {
        sessionContext.AbortTransaction(sessionContext)
        return err
    }

    return sessionContext.CommitTransaction(sessionContext)
})

```
