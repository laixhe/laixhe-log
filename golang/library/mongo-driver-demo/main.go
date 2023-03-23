package main

import (
	"context"
	"fmt"
	"log"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
	"go.mongodb.org/mongo-driver/x/bsonx"
)

type Mgo struct {
	Database *mongo.Database
}

func NewMgo() *Mgo {

	// 单机版
	// mongodb://127.0.0.1:27017
	// mongodb://root:123456@127.0.0.1:27017
	// mongodb://root:123456@127.0.0.1:27017?authSource=admin&tls=true&tlsInsecure=true&maxPoolSize=100&minPoolSize=10&connectTimeoutMS=5000
	//
	// 副本集
	// mongodb://root:123456@127.0.0.1:27017,127.0.0.1:27018,127.0.0.1:27019/?replicaSet=myRepl

	opts := options.Client()
	opts.ApplyURI("mongodb://127.0.0.1:27017")

	// 进行配置
	//opts.SetMaxPoolSize(100)                 // 设置最大连接的数量
	//opts.SetConnectTimeout(5 * time.Second)  // 设置连接超时时间 5 秒
	//opts.SetMaxConnIdleTime(5 * time.Second) // 设置连接的空闲时间 5 秒
	//opts.SetAuth(options.Credential{// 认证参数设置，否则连不上
	//	AuthMechanism: "SCRAM-SHA-1",
	//	AuthSource:    "test",
	//	Username:      "laixhe",
	//	Password:      "123456"})
	// 上面所有配置都是可以 URI 进行配置的

	// 链接 mongo 服务
	client, err := mongo.Connect(context.Background(), opts)
	if err != nil {
		log.Fatalln("Connect err:", err)
	}

	// 判断服务是否可用
	err = client.Ping(context.Background(), readpref.Primary())
	if err != nil {
		log.Fatalln("Ping err:", err)
	}

	// 断开连接
	//defer client.Disconnect(context.Background())

	// 指定的数据库
	// use test
	db := client.Database("test")

	// 选择集合(表)
	//user := db.Collection("user")
	// 删除这个集合
	//user.Drop(context.Background())

	return &Mgo{Database: db}
}

type User struct {
	Id    primitive.ObjectID `bson:"_id"`
	Name  string             `bson:"name"`
	Age   int                `bson:"age"`
	Items []Item             `bson:"items"`
	Like  []int              `bson:"like"`
}

type Item struct {
	Code string                 `bson:"code"`
	Data map[string]interface{} `bson:"data"`
}

func (u *User) TableName() string {
	return "user"
}

// CreateIndex 设置索引
func (m *Mgo) CreateIndex() error {

	indexOptions := options.Index()
	indexOptions.SetExpireAfterSeconds(1 * 24 * 3600)
	indexOptions.SetUnique(true)

	indexModel := mongo.IndexModel{
		Keys:    bsonx.Doc{{"name", bsonx.Int32(1)}},
		Options: indexOptions,
	}

	indexString, err := m.Database.Collection("user").Indexes().CreateOne(context.Background(), indexModel)

	fmt.Println("index:", indexString)
	return err
}

// UserInsertOne 插入一条数据
func (m *Mgo) UserInsertOne(user *User) (*mongo.InsertOneResult, error) {
	return m.Database.Collection(user.TableName()).InsertOne(context.Background(), user)
}

// UserInsertMany 插入多条数据
func (m *Mgo) UserInsertMany(users []User) (*mongo.InsertManyResult, error) {
	documents := make([]interface{}, 0, len(users))
	for k := range users {
		documents = append(documents, users[k])
	}
	return m.Database.Collection(users[0].TableName()).InsertMany(context.Background(), documents)
}

// UserFindOne 查询单条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? AND `age`=? LIMIT 1
func (m *Mgo) UserFindOne(name string, age int) (*User, error) {
	// 查询操作
	// bson.D 是顺序查询
	filter := bson.D{}
	filter = append(filter, bson.E{Key: "name", Value: name})
	filter = append(filter, bson.E{Key: "age", Value: age})
	// bson.M 是顺序无关查询
	//filter := bson.M{"name": name, "age": age}
	// 上述即是 有序 与 无序 区别

	user := new(User)
	err := m.Database.Collection(user.TableName()).FindOne(context.Background(), filter).Decode(user)
	if err != nil {
		return nil, err
	}
	return user, nil
}

// UserFind 一次查询多条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `age`>=? ORDER BY age DESC LIMIT ?,2
func (m *Mgo) UserFind(age int, page int64) ([]User, error) {
	// 每页取 2 条
	var limit int64 = 2
	if page <= 1 {
		page = 0
	} else {
		page = limit * (page - 1)
	}

	// 查询操作
	filter := bson.M{"age": bson.M{"$gte": age}}

	findOptions := options.Find()
	findOptions.SetSort(bson.M{"age": -1}) // 排序 1=升序asc -1=降序desc
	findOptions.SetSkip(page)              // 跳过
	findOptions.SetLimit(limit)            // 读取数量

	// 查询
	user := new(User)
	cursor, err := m.Database.Collection(user.TableName()).Find(context.Background(), filter, findOptions)
	if err != nil {
		return nil, err
	}
	if cursor.Err() != nil {
		return nil, cursor.Err()
	}
	defer cursor.Close(context.Background())

	// 获取数据(一次性)
	users := make([]User, 0, limit)
	err = cursor.All(context.Background(), &users)
	if err != nil {
		return nil, err
	}
	// 获取数据(迭代)
	//for cursor.Next(context.Background()) {
	//	var user User
	//	if err = cursor.Decode(&user); err != nil {
	//		return nil, err
	//	}
	//	users = append(users, user)
	//}
	return users, nil
}

// UserFindLike 模糊查询多条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name` LIKE '%xxx%'
func (m *Mgo) UserFindLike(name string) ([]User, error) {
	// 查询操作
	filter := bson.M{"name": primitive.Regex{Pattern: name}}

	// 查询
	user := new(User)
	cursor, err := m.Database.Collection(user.TableName()).Find(context.Background(), filter)
	if err != nil {
		return nil, err
	}
	if cursor.Err() != nil {
		return nil, cursor.Err()
	}
	defer cursor.Close(context.Background())

	// 获取数据
	users := make([]User, 0, 2)
	err = cursor.All(context.Background(), &users)
	if err != nil {
		return nil, err
	}

	return users, nil
}

// UserFindOneAndDelete 查询单条数据后删除该数据，返回删除前的数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? LIMIT 1
// DELETE FROM `user` WHERE `name`=? LIMIT 1
func (m *Mgo) UserFindOneAndDelete(name string) (*User, error) {
	// 查询操作
	filter := bson.M{"name": name}

	user := new(User)
	err := m.Database.Collection(user.TableName()).FindOneAndDelete(context.Background(), filter).Decode(user)
	if err != nil {
		return nil, err
	}
	return user, nil
}

// UserFindOneAndUpdate 查询单条数据后修改该数据，返回修改前的数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? LIMIT 1
// UPDATE `user` SET `name`=? WHERE `name`=? LIMIT 1
func (m *Mgo) UserFindOneAndUpdate(name, mName string) (*User, error) {
	// 查询单条数据
	user := new(User)
	err := m.Database.Collection(user.TableName()).FindOneAndUpdate(context.Background(),
		bson.M{"name": name},                   // 查询操作
		bson.M{"$set": bson.M{"name": mName}}). // 修改操作
		Decode(user)
	if err != nil {
		return nil, err
	}
	return user, nil
}

// 查询单条数据后替换该数据(以前的数据全部清空)
//m.Database.Collection("user").FindOneAndReplace(context.Background(), bson.D{}, bson.M{}).Decode(user)

// UserCount 查询 count 总数
// SELECT COUNT(*) FROM `user`
func (m *Mgo) UserCount() (int64, error) {
	user := new(User)
	return m.Database.Collection(user.TableName()).EstimatedDocumentCount(context.Background())
}

// UserWhereCount 按条件查询 count 总数
// SELECT COUNT(*) FROM `user` WHERE `age`>=?
func (m *Mgo) UserWhereCount(age int) (int64, error) {
	// 查询操作
	filter := bson.M{"age": bson.M{"$gte": age}}
	user := new(User)
	return m.Database.Collection(user.TableName()).CountDocuments(context.Background(), filter)
}

// UserUpdateOne 修改一条数据
// UPDATE `user` SET `name`=? WHERE `name`=? LIMIT 1
func (m *Mgo) UserUpdateOne(name, mName string) error {
	user := new(User)
	result, err := m.Database.Collection(user.TableName()).UpdateOne(context.Background(),
		bson.M{"name": name},                  // 查询操作
		bson.M{"$set": bson.M{"name": mName}}) // 修改操作
	if err != nil {
		return err
	}
	if result.MatchedCount == result.ModifiedCount {
	}
	return nil
}

// UserUpdateMany 修改多条数据
// UPDATE `user` SET `name`=? WHERE `name`=?
func (m *Mgo) UserUpdateMany(name, mName string) error {
	user := new(User)
	_, err := m.Database.Collection(user.TableName()).UpdateMany(context.Background(),
		bson.M{"name": name},                  // 查询操作
		bson.M{"$set": bson.M{"name": mName}}) // 修改操作
	return err
}

// UserDeleteOne 删除一条数据
// DELETE FROM `user` WHERE `name`=? LIMIT 1
func (m *Mgo) UserDeleteOne(name string) error {
	user := new(User)
	result, err := m.Database.Collection(user.TableName()).DeleteOne(context.Background(),
		bson.M{"name": name}) // 查询操作
	if err != nil {
		return err
	}
	if result.DeletedCount > 0 {
	}
	return nil
}

// UserDeleteMany 删除多条数据
// DELETE FROM `user` WHERE `name`=?
func (m *Mgo) UserDeleteMany(name string) error {
	user := new(User)
	result, err := m.Database.Collection(user.TableName()).DeleteMany(context.Background(),
		bson.M{"name": name}) // 查询操作
	if err != nil {
		return err
	}
	if result.DeletedCount > 0 {
	}
	return nil
}

type UserGroup struct {
	Age      int `bson:"_id"`
	CountAge int `bson:"count_age"`
}

// UserAggregateGroup 按 name 排序并统计 age 总和 (固定写法且 _id 不能少)
// db.user.aggregate([{$group:{_id:"$age", age:{$sum:"$age"}}}])
// SELECT `age` AS `_id`, SUM(`age`) AS `count_age` FROM `user` GROUP BY `age`
func (m *Mgo) UserAggregateGroup() ([]UserGroup, error) {

	groupStage := mongo.Pipeline{
		bson.D{
			{"$group", bson.D{
				{"_id", "$age"},
				{"count_age", bson.D{{"$sum", "$age"}}},
			},
			},
		},
	}

	user := new(User)
	cursor, err := m.Database.Collection(user.TableName()).Aggregate(context.Background(), groupStage)
	if err != nil {
		return nil, err
	}
	if cursor.Err() != nil {
		return nil, cursor.Err()
	}
	defer cursor.Close(context.Background())

	// 获取数据
	userGroups := make([]UserGroup, 0, 10)
	err = cursor.All(context.Background(), &userGroups)
	if err != nil {
		return nil, err
	}

	return userGroups, nil
}

// UserAggregateGroupWhere 统计 age 小于等于 19 的条数
// db.user.aggregate([{$match:{age:{$lte:19}}},{$group:{_id:"$age", age:{$sum:1}}}])
// SELECT 'age' AS `_id`, SUM(1) AS `count_age` FROM `user` WHERE `age`<=19
func (m *Mgo) UserAggregateGroupWhere() ([]UserGroup, error) {

	match := bson.M{
		"age": bson.M{"$lte": 19},
	}

	group := bson.D{
		{"_id", "$age"},
		{"count_age", bson.M{"$sum": 1}},
	}

	groupStage := mongo.Pipeline{
		bson.D{
			{"$match", match},
		},
		bson.D{
			{"$group", group},
		}}

	user := new(User)
	cursor, err := m.Database.Collection(user.TableName()).Aggregate(context.Background(), groupStage)
	if err != nil {
		return nil, err
	}
	if cursor.Err() != nil {
		return nil, cursor.Err()
	}
	defer cursor.Close(context.Background())

	// 获取数据
	userGroups := make([]UserGroup, 0, 10)
	err = cursor.All(context.Background(), &userGroups)
	if err != nil {
		return nil, err
	}

	return userGroups, nil
}
