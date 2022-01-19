package main

import (
	"fmt"
	"testing"
)

func TestCreate(t *testing.T) {
	initDb()
	AutoMigrate()
	u := &User{
		Name:  "laixhe",
		Age:   18,
		Score: 10.1,
		Is:    false,
	}

	err := Create(u)
	if err != nil {
		fmt.Println("Create err", err)
		return
	}
	fmt.Println("Create", u)
}

func TestGet(t *testing.T) {
	initDb()

	u := &User{
		Name:  "laixhe",
	}

	err := Get(u)
	fmt.Println("Get", u)
	fmt.Println("Get err", err)
}

func TestGetName(t *testing.T) {
	initDb()

	u := &User{
		Name:  "laixhe",
	}

	err := GetName(u)
	fmt.Println("GetName", u)
	fmt.Println("GetName err", err)
}

func TestSelect(t *testing.T) {
	initDb()

	data, err := Select()
	fmt.Println("Select", data)
	fmt.Println("Select err", err)
}

func TestSelectName(t *testing.T) {
	initDb()

	data, err := SelectName()
	fmt.Println("SelectName", data)
	fmt.Println("SelectName err", err)
}

func TestUpdate(t *testing.T) {
	initDb()

	u := &User{
		Name:  "laixhe",
		Age:   18,
		Score: 18.20,
	}

	err := Update(u)
	fmt.Println("Update", u)
	fmt.Println("Update err", err)
}

func TestSave(t *testing.T) {
	initDb()

	u := &User{
		ID: 10,
		Name:  "laixhe01",
		Age:   20,
		Score: 20.3,
		Is: true,
	}

	err := Save(u)
	fmt.Println("Save", u)
	fmt.Println("Save err", err)
}

func TestExpr(t *testing.T) {
	initDb()
	u := &User{
		ID: 10,
	}

	err := Expr(u)
	fmt.Println("Expr", u)
	fmt.Println("Expr err", err)
}

func TestExprs(t *testing.T) {
	initDb()
	u := &User{
		ID: 10,
	}

	err := Exprs(u)
	fmt.Println("Exprs", u)
	fmt.Println("Exprs err", err)
}
