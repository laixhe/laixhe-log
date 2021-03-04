package main

import (
	"fmt"
	"log"
	"time"

	_ "github.com/go-sql-driver/mysql"
	"github.com/jmoiron/sqlx"
)

type SQLXDB struct {
	X *sqlx.DB
}

func initDb() *SQLXDB {

	//x, err := sqlx.Connect("sqlite3", "./db/demo.db")
	dsn := "root:123456@tcp(127.0.0.1:3306)/test?charset=utf8mb4&collation=utf8mb4_unicode_ci&parseTime=True&loc=Local"
	db, err := sqlx.Connect("mysql", dsn)
	if err != nil {
		log.Fatal("Connect:", err)
	}

	db.SetMaxIdleConns(10)                 // 设置闲置的连接数
	db.SetMaxOpenConns(100)                // 设置最大打开的连接数
	db.SetConnMaxLifetime(5 * time.Minute) // 设置了连接可复用的最大时间(要比数据库设置连接超时时间少)

	// 验证数据库连接是否正常
	err = db.Ping()
	if err != nil {
		log.Fatal("Ping:", err)
	}

	return &SQLXDB{
		X: db,
	}
}

type User struct {
	Id   int    `db:"id"`
	Name string `db:"name"`
	Age  int    `db:"age"`
}

// 获取字段为 NULL 时，需使用下面代码
//type User struct {
//	Id   int            `db:"id"`
//	Name sql.NullString `db:"name"`
//	Age  sql.NullInt32  `db:"age"`
//}

// 单行查询
func (this *SQLXDB) Get(name string) {

	user := new(User)

	query := "SELECT `id`,`name`,`age` FROM `user` WHERE `name`=?"

	err := this.X.Get(user, query, name)
	if err != nil {
		fmt.Printf("Get: err: %v\n", err)
		return
	}

	fmt.Printf("Get: id=%d name=%s age=%d\n", user.Id, user.Name, user.Age)

}

// 多行查询
func (this *SQLXDB) Select() {

	users := make([]User, 0, 5)

	query := "SELECT `id`,`name`,`age` FROM `user`"

	err := this.X.Select(&users, query)
	if err != nil {
		fmt.Printf("Select: err: %v\n", err)
		return
	}

	fmt.Printf("Select: %v\n", users)
}

// 插入、更新、删除
func (this *SQLXDB) Exec(name string, age int) {

	query := "INSERT INTO `user`(`name`,`age`) VALUES(?,?)"

	result, err := this.X.Exec(query, name, age)
	if err != nil {
		fmt.Printf("Exec: err: %v\n", err)
		return
	}

	// 获取自增ID
	id, err := result.LastInsertId()
	if err != nil {
		fmt.Printf("Exec: LastInsertId err: %v\n", err)
		return
	}

	// 获取执行(影响)的行数
	//result.RowsAffected()

	fmt.Printf("Exec: id=%d\n", id)

}

// IN语句的支持处理
func (this *SQLXDB) In(names []string) {

	var users []User

	query := "SELECT `id`,`name`,`age` FROM `user` WHERE `name` IN(?)"

	query, args, err := sqlx.In(query, names)
	if err != nil {
		fmt.Printf("In: sqlx.In err: %v\n", err)
		return
	}

	// 第一个参数，string，是处理完后的 sql 语句，其中的 In 查询语句中的一个 ? 已经按照实际的 list 长度进行处理，替换为多个 ?
	// 第二个参数，[]interface{}，查询参数列表
	// 第三个参数，error，错误对象
	fmt.Printf("In: sqlx.In query: %v\n", query)
	fmt.Printf("In: sqlx.In args: %v\n", args)

	err = this.X.Select(&users, query, args...)
	if err != nil {
		fmt.Printf("In: Select err: %v\n", err)
		return
	}

	fmt.Printf("In: %v\n", users)

}

// 事务
func (this *SQLXDB) Transaction() {

	// 开启事务
	tx := this.X.MustBegin()

	// 执行
	_, err := tx.Exec("UPDATE `user` SET `name`='laixhe3' WHERE `age`=18")
	if err != nil {
		// 回滚事务
		tx.Rollback()

		fmt.Printf("Transaction: Exec err: %v\n", err)
		return
	}

	_, err = tx.Exec("INSERT INTO `user`(`name`,`age`) VALUES('laixhe4', 21)")
	if err != nil {
		// 回滚事务
		tx.Rollback()

		fmt.Printf("Transaction: Exec err: %v\n", err)
		return
	}

	// 提交事务
	err = tx.Commit()
	if err != nil {
		// 回滚事务
		tx.Rollback()

		fmt.Printf("Transaction: Exec err: %v\n", err)
		return
	}

	fmt.Printf("Transaction: %v\n", "ok")
}
