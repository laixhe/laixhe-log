package main

import (
	"database/sql"
	"fmt"
	"time"

	_ "github.com/go-sql-driver/mysql"
)

var DB *sql.DB

func main() {

	err := initDb()
	if err != nil {
		fmt.Printf("init db failed, err: %v\n", err)
		return
	}

	Exec()
	Get(1)
	Select(0)
	//UpdatePrepare(3)
	//SelectPrepare(0)
	//Trans()

}

func initDb() error {

	var err error
	dsn := "root:123456@tcp(127.0.0.1:3306)/test?charset=utf8mb4&collation=utf8mb4_unicode_ci&parseTime=True&loc=Local"

	DB, err = sql.Open("mysql", dsn)
	if err != nil {
		return err
	}
	//defer db.Close()

	// 设置最大-打开的连接数
	DB.SetMaxOpenConns(100)
	// 设置最大-空闲的连接数
	DB.SetMaxIdleConns(10)
	// 设置了连接可复用的最大时间(要比数据库设置连接超时时间少)
	DB.SetConnMaxLifetime(5 * time.Minute)

	// 验证数据库连接是否正常
	err = DB.Ping()
	if err != nil {
		return err
	}

	return nil
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
func Get(id int) {

	query := "SELECT `id`,`name`,`age` FROM `user` WHERE `id`=?"

	// 执行查询
	row := DB.QueryRow(query, id)

	var user User

	// 获取的顺序一定要和 SELECT 取出的顺序一样
	err := row.Scan(&user.Id, &user.Name, &user.Age)
	if err != nil {
		fmt.Printf("Scan failed, err: %v\n", err)
		return
	}

	fmt.Printf("Get: id=%d name=%s age=%d\n", user.Id, user.Name, user.Age)

}

// 多行查询
func Select(id int) {

	query := "SELECT `id`,`name`,`age` FROM `user` WHERE `id`>?"

	// 执行查询
	rows, err := DB.Query(query, id)
	if err != nil {
		fmt.Printf("Query failed, err: %v\n", err)
		return
	}
	defer rows.Close()

	var users = make([]User, 0, 5)

	for rows.Next() {

		var user User

		// 获取的顺序一定要和 SELECT 取出的顺序一样
		err := rows.Scan(&user.Id, &user.Name, &user.Age)
		if err != nil {
			fmt.Printf("Next Scan failed, err: %v\n", err)
			return
		}

		users = append(users, user)

	}

	fmt.Printf("Select: %v\n", users)

}

// 插入、更新、删除
func Exec() {

	query := "INSERT INTO `user`(`name`,`age`) VALUES(?,?)"

	result, err := DB.Exec(query, "test", 20)
	if err != nil {
		fmt.Printf("Exec failed, err: %v\n", err)
		return
	}

	// 获取自增ID
	id, err := result.LastInsertId()
	if err != nil {
		fmt.Printf("LastInsertId failed, err: %v\n", err)
		return
	}

	// 获取执行(影响)的行数
	//result.RowsAffected()

	fmt.Printf("Exec id=%d\n", id)
}

// 预处理

// 查询-预处理
func SelectPrepare(id int) {

	query := "SELECT `id`,`name`,`age` FROM `user` WHERE `id`>?"

	// 预处理
	stmt, err := DB.Prepare(query)
	if err != nil {
		fmt.Printf("SelectPrepare Prepare failed, err: %v\n", err)
		return
	}
	defer stmt.Close()

	// 执行查询
	rows, err := stmt.Query(id)
	if err != nil {
		fmt.Printf("SelectPrepare Query failed, err: %v\n", err)
		return
	}
	defer rows.Close()

	var users = make([]User, 0, 5)

	for rows.Next() {

		var user User

		// 获取的顺序一定要和 SELECT 取出的顺序一样
		err := rows.Scan(&user.Id, &user.Name, &user.Age)
		if err != nil {
			fmt.Printf("SelectPrepare Next Scan failed, err: %v\n", err)
			return
		}

		users = append(users, user)

	}

	fmt.Printf("SelectPrepare: %v\n", users)

}

// 修改-预处理
func UpdatePrepare(id int) {

	query := "UPDATE `user` SET `name`=? WHERE `id`=?"

	// 预处理
	stmt, err := DB.Prepare(query)
	if err != nil {
		fmt.Printf("UpdatePrepare Prepare failed, err: %v\n", err)
		return
	}
	defer stmt.Close()

	result, err := stmt.Exec("-update-", id)
	if err != nil {
		fmt.Printf("UpdatePrepare Exec failed, err: %v\n", err)
		return
	}

	// 获取执行(影响)的行数
	affected, err := result.RowsAffected()
	if err != nil {
		fmt.Printf("LastInsertId failed, err: %v\n", err)
		return
	}

	fmt.Printf("UpdatePrepare Exec affected=%d\n", affected)

}

// 事务

func Trans() {

	// 开启事务
	conn, err := DB.Begin()
	if err != nil {

		if conn != nil {
			// 出错回滚 事务
			conn.Rollback()
		}

		fmt.Printf("begin faile, err: %v\n", err)
		return
	}

	query := "UPDATE `user` SET `age`=`age`+1 WHERE `id`=?"
	_, err = conn.Exec(query, 1)
	if err != nil {
		conn.Rollback()
		fmt.Printf("exec sql: %s failed, err: %v\n", query, err)
		return
	}

	query = "UPDATE `user` SET `name`='-name-' WHERE `id`=?"
	_, err = conn.Exec(query, 2)
	if err != nil {
		conn.Rollback()
		fmt.Printf("exec sql: %s failed, err: %v\n", query, err)
		return
	}

	// 提交事务
	err = conn.Commit()
	if err != nil {
		conn.Rollback()
		fmt.Printf("Commit failed, err: %v\n", err)
		return
	}

	fmt.Println("Commit ok")

}
