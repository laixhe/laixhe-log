package main

import (
	"context"
	"fmt"
	"log"
	"time"

	"go.mongodb.org/mongo-driver/bson"
	"go.mongodb.org/mongo-driver/bson/primitive"
	"go.mongodb.org/mongo-driver/mongo"
	"go.mongodb.org/mongo-driver/mongo/options"
	"go.mongodb.org/mongo-driver/mongo/readpref"
	"go.mongodb.org/mongo-driver/x/bsonx"
)

type User struct {
	Id          primitive.ObjectID   `bson:"_id"`
	Name        string               `bson:"name"`
	Age         int                  `bson:"age"`
	AsStr       []string             `bson:"as_str"`
	AsInt       []int                `bson:"as_int"`
	AsStruct    []AsTStruct          `bson:"as_struct"`
	AsMap       map[string]string    `bson:"as_map"`
	AsMapStruct map[string]AsTStruct `bson:"as_map_struct"`
}

type AsTStruct struct {
	Tint int    `bson:"tint"`
	Tstr string `bson:"tstr"`
}

type Mgo struct {
	user *mongo.Collection
}

func NewMgo() *Mgo {

	opts := options.Client()
	opts.ApplyURI("mongodb://127.0.0.1:27017") // mongodb://root:123456@127.0.0.1:27017
	opts.SetMaxPoolSize(100)                   // 设置最大连接的数量
	opts.SetConnectTimeout(5 * time.Second)    // 设置连接超时时间 5 秒
	opts.SetMaxConnIdleTime(5 * time.Second)   // 设置连接的空闲时间 5 秒

	//认证参数设置，否则连不上
	//opts.SetAuth(options.Credential{
	//	AuthMechanism: "SCRAM-SHA-1",
	//	AuthSource:    "test",
	//	Username:      "laixhe",
	//	Password:      "123456"})

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
	user := db.Collection("user")
	// 删除这个集合
	//user.Drop(context.Background())

	return &Mgo{user: user}
}

// 设置索引
func (m *Mgo) UserCreateIndex() error {

	indexOptions := options.Index()
	indexOptions.SetExpireAfterSeconds(1 * 24 * 3600)
	indexOptions.SetUnique(true)

	indexModel := mongo.IndexModel{
		Keys:    bsonx.Doc{{"name", bsonx.Int32(1)}},
		Options: indexOptions,
	}

	indexString, err := m.user.Indexes().CreateOne(context.Background(), indexModel)

	fmt.Println("index:", indexString)

	return err
}

// 插入一条数据
func (m *Mgo) UserInsertOne(data *User) (*mongo.InsertOneResult, error) {

	result, err := m.user.InsertOne(context.Background(), data)
	if err != nil {
		return nil, err
	}

	return result, nil
}

// 插入多条数据
//m.user.InsertMany(context.Background(), []*User{});

// 查询单条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? AND `age`=? LIMIT 1
func (m *Mgo) UserFindOne(name string, age int) (*User, error) {

	// 查询单条数据
	user := new(User)

	// where
	where := bson.M{"name": name, "age": age}

	err := m.user.FindOne(context.Background(), where).Decode(user)

	if err != nil {
		return nil, err
	}

	return user, nil
}

// 一次查询多条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `age`>=? ORDER BY age DESC LIMIT ?,2
func (m *Mgo) UserFind(age int, page int64) ([]User, error) {

	// 每页取 2 条
	if page <= 1 {
		page = 0
	} else {
		page = 2 * (page - 1)
	}

	// where
	where := bson.M{"age": bson.M{"$gte": age}}

	findOptions := options.Find()
	findOptions.SetSort(bson.M{"age": -1}) // 排序 1=升序asc、-1=降序desc
	findOptions.SetSkip(page)              // 跳过
	findOptions.SetLimit(2)                // 读取数量

	// 查询
	cursor, err := m.user.Find(context.Background(), where, findOptions)
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

	//for cursor.Next(context.Background()) {
	//	var user User
	//	if err = cursor.Decode(&user); err != nil {
	//		return nil, err
	//	}
	//	users = append(users, user)
	//}

	return users, nil
}

// 模糊查询多条数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name` LIKE '%xxx%'
func (m *Mgo) UserFindLike(name string) ([]User, error) {

	// where
	where := bson.M{"name": primitive.Regex{Pattern: name}}

	// 查询
	cursor, err := m.user.Find(context.Background(), where)
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

// 查询单条数据后删除该数据，返回删除前的数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? LIMIT 1
// DELETE FROM `user` WHERE `name`=? LIMIT 1
func (m *Mgo) UserFindOneAndDelete(name string) (*User, error) {

	// 查询单条数据
	user := new(User)

	// where
	where := bson.M{"name": name}

	err := m.user.FindOneAndDelete(context.Background(), where).Decode(user)
	if err != nil {
		return nil, err
	}

	return user, nil
}

// 查询单条数据后修改该数据，返回修改前的数据
// 没有查询到数据返回错误
// SELECT * FROM `user` WHERE `name`=? LIMIT 1
// UPDATE `user` SET `name`=? WHERE `name`=? LIMIT 1
func (m *Mgo) UserFindOneAndUpdate(name, mName string) (*User, error) {

	// 查询单条数据
	user := new(User)
	err := m.user.FindOneAndUpdate(context.Background(),
		bson.M{"name": name},                   // where
		bson.M{"$set": bson.M{"name": mName}}). // update
		Decode(user)
	if err != nil {
		return nil, err
	}

	return user, nil
}

// 查询单条数据后替换该数据(以前的数据全部清空)
//m.user.FindOneAndReplace(context.Background(), bson.D{}, bson.M{}).Decode(user)

// 查询 count 总数
// SELECT COUNT(*) FROM `user`
func (m *Mgo) UserCount() (int64, error) {
	size, err := m.user.EstimatedDocumentCount(context.Background())
	return size, err

}

// 按条件查询 count 总数
// SELECT COUNT(*) FROM `user` WHERE `age`>=?
func (m *Mgo) UserWhereCount(age int) (int64, error) {

	// where
	where := bson.M{"age": bson.M{"$gte": age}}
	size, err := m.user.CountDocuments(context.Background(), where)

	return size, err
}

// 修改一条数据
// UPDATE `user` SET `name`=? WHERE `name`=? LIMIT 1
func (m *Mgo) UserUpdateOne(name, mName string) error {

	result, err := m.user.UpdateOne(context.Background(),
		bson.M{"name": name},                  // where
		bson.M{"$set": bson.M{"name": mName}}) // update
	if err != nil {
		return err
	}

	if result.MatchedCount == result.ModifiedCount {
	}

	return nil
}

// 修改多条数据
// UPDATE `user` SET `name`=? WHERE `name`=?
func (m *Mgo) UserUpdateMany(name, mName string) error {

	_, err := m.user.UpdateMany(context.Background(),
		bson.M{"name": name},                  // where
		bson.M{"$set": bson.M{"name": mName}}) // update
	if err != nil {
		return err
	}

	return nil
}

// 删除一条数据
// DELETE FROM `user` WHERE `name`=? LIMIT 1
func (m *Mgo) UserDeleteOne(name string) error {

	result, err := m.user.DeleteOne(context.Background(),
		bson.M{"name": name}) // where
	if err != nil {
		return err
	}

	if result.DeletedCount > 0 {
	}

	return nil
}

// 删除多条数据
// DELETE FROM `user` WHERE `name`=?
func (m *Mgo) UserDeleteMany(name string) error {

	result, err := m.user.DeleteMany(context.Background(),
		bson.M{"name": name}) // where
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

// 按 name 排序并统计 age 总和 (固定写法且 _id 不能少)
// db.user.aggregate([{$group:{_id:"$age", age:{$sum:"$age"}}}])
// SELECT `age` AS `_id`, SUM(`age`) AS `count_age` FROM `user` GROUP BY `age`
func (m *Mgo) UserAggregateGroup() ([]UserGroup, error) {

	groupStage := mongo.Pipeline{bson.D{
		{"$group", bson.D{
			{"_id", "$age"},
			{"count_age", bson.D{{"$sum", "$age"}}},
		},
		},
	}}

	cursor, err := m.user.Aggregate(context.Background(), groupStage)
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

// 统计 age 小于等于 19 的条数
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

	cursor, err := m.user.Aggregate(context.Background(), groupStage)
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
