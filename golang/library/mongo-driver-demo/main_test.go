package main

import (
	"fmt"
	"testing"

	"go.mongodb.org/mongo-driver/bson/primitive"
)

func TestMgo_UserInsertOne(t *testing.T) {

	user := &User{
		Id: primitive.NewObjectID(),
		Name: "laixhe",
		Age:  18,
		As:   []string{"aa", "dd"},
	}

	result, err := NewMgo().UserInsertOne(user)

	fmt.Printf("result=%s err=%v\n", result.InsertedID, err)
}

func TestMgo_UserFindOne(t *testing.T) {
	user, err := NewMgo().UserFindOne("laixhe", 18)
	fmt.Printf("user=%v err=%v\n", user, err)
}

func TestMgo_UserFind(t *testing.T) {
	users, err := NewMgo().UserFind(19, 1)
	fmt.Printf("users=%v err=%v\n", users, err)
}

func TestMgo_UserFindLike(t *testing.T) {
	users, err := NewMgo().UserFindLike("laixhe")
	fmt.Printf("users=%v err=%v\n", users, err)
}

func TestMgo_UserFindOneAndDelete(t *testing.T) {
	user, err := NewMgo().UserFindOneAndDelete("laiki")
	fmt.Printf("user=%v err=%v\n", user, err)
}

func TestMgo_UserFindOneAndUpdate(t *testing.T) {
	user, err := NewMgo().UserFindOneAndUpdate("laiki", "laixhe000")
	fmt.Printf("user=%v err=%v\n", user, err)
}

func TestMgo_UserCount(t *testing.T) {
	count, err := NewMgo().UserCount()
	fmt.Printf("count=%v err=%v\n", count, err)
}

func TestMgo_UserWhereCount(t *testing.T) {
	count, err := NewMgo().UserWhereCount(19)
	fmt.Printf("count=%v err=%v\n", count, err)
}

func TestMgo_UserUpdateOne(t *testing.T) {
	err := NewMgo().UserUpdateOne("laiki", "laixhe")
	fmt.Printf("err=%v\n", err)
}

func TestMgo_UserUpdateMany(t *testing.T) {
	err := NewMgo().UserUpdateMany("laiki", "laixhe")
	fmt.Printf("err=%v\n", err)
}

func TestMgo_UserDeleteOne(t *testing.T) {
	err := NewMgo().UserDeleteOne("laiki")
	fmt.Printf("err=%v\n", err)
}

func TestMgo_UserDeleteMany(t *testing.T) {
	err := NewMgo().UserDeleteMany("laiki")
	fmt.Printf("err=%v\n", err)
}

func TestMgo_UserAggregateGroup(t *testing.T) {
	userGroups, err := NewMgo().UserAggregateGroup()
	fmt.Printf("userGroups=%v err=%v\n", userGroups, err)
}

func TestMgo_UserAggregateGroupWhere(t *testing.T) {
	userGroups, err := NewMgo().UserAggregateGroupWhere()
	fmt.Printf("userGroups=%v err=%v\n", userGroups, err)
}
