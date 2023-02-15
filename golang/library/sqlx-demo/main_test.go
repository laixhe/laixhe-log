package main

import "testing"

func TestSQLXDB_Exec(t *testing.T) {
	x := initDb()
	x.Exec("laixhe0", 18)
	x.Exec("laixhe1", 19)
	x.Exec("laixhe2", 20)
}

func TestSQLXDB_Get(t *testing.T) {
	x := initDb()
	x.Get("laixhe1")
}

func TestSQLXDB_Select(t *testing.T) {
	x := initDb()
	x.Select()
}

func TestSQLXDB_In(t *testing.T) {
	x := initDb()
	x.In([]string{"laixhe1", "laixhe2"})
}

func TestSQLXDB_Transaction(t *testing.T) {
	x := initDb()
	x.Transaction()
}