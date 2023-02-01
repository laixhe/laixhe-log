# MongoDB 官方驱动

> 安装

- go get go.mongodb.org/mongo-driver

> 导入

- import "go.mongodb.org/mongo-driver/mongo"

```
find()   # 查询多条数据
update() # 修改(更新)
save()   # 不存在则插入，存在则修改(更新)


# 修改字段值，不存在则新增该字段 #如: { $set: { <field>: <value1> }}
$set
# 删除字段 {"$unset": {"要删除的字段名": 1}}
$unset
# 自增自减
#inc

# 数组追加元素，字段不存在则新增 #如: { $push: { <field>: <value1> } } 
$push 
# 数组删除元素，如果有多个相同元素，会全都被删除
$pull
# 数组删除多个元素
$pullAll
# 数组删除元素，不过它删除的是最后一个或第一个元素，值只能选 1 或 -1
$pop
# 追加数据时避免重复
$addToSet
# 添加多个数据,适用于 $addToSet 操作符和 $push 操作符
$each 
## 如: { $push: { <field>: { $each: [ <value1>, <value2> ... ] } } }
## 如: { $addToSet: { <field>: { $each: [ <value1>, <value2> ... ] } } }
# 不在....里面
$ne

# 包含，与 mysql 的 in 功能一样
$in
# 不包含
$nin
# 判断字段是否存在
$exists #如: {name: {$exists: true}}

# 大于
$gt
# 小于
$lt
# 大于等于
$gte
# 小于等于
$lte
# 不等于
$ne
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
