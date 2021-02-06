package main

import (
	"fmt"
	"log"
	"time"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var db *gorm.DB

// initDb 初始化数据库
func initDb() {

	dsn := "root:123456@tcp(127.0.0.1:3306)/test?charset=utf8mb4&collation=utf8mb4_unicode_ci&parseTime=True&loc=Local"
	var err error
	db, err = gorm.Open(mysql.Open(dsn), &gorm.Config{
		Logger: logger.Default.LogMode(logger.Info),
	})
	if err != nil {
		log.Fatal("Open:", err)
	}

	sqlDB, err := db.DB()
	if err != nil {
		log.Fatal("db:", err)
	}

	// SetMaxIdleConns 设置空闲连接池中连接的最大数量
	sqlDB.SetMaxIdleConns(10)

	// SetMaxOpenConns 设置打开数据库连接的最大数量
	sqlDB.SetMaxOpenConns(100)

	// SetConnMaxLifetime 设置了连接可复用的最大时间
	sqlDB.SetConnMaxLifetime(time.Hour)

	// 验证数据库连接是否正常
	err = sqlDB.Ping()
	if err != nil {
		log.Fatal("ping:", err)
	}

}

// User 用户表
type User struct {
	ID        int     `gorm:"column:id;type:uint;not null;primaryKey;autoIncrement;comment:用户ID自增"`
	Name      string  `gorm:"column:name;type:string;size:100;not null;default:'';comment:用户名"`
	Age       int     `gorm:"column:age;type:uint;size:9;not null;default:0;comment:年龄"`                //【tinyint size:8】【smallint size:9】【int size:25】【bigint size:33】  ??? (为什么)
	Score     float64 `gorm:"column:score;type:decimal(5,2) unsigned;not null;default:0.00;comment:分数"` // decimal ???
	Is        bool    `gorm:"column:is;type:bool;not null;default:0;comment:测试bool"`                    // mysql 自动转换 0<->false 1<->true
	CreatedAt time.Time
	UpdatedAt time.Time
	DeletedAt gorm.DeletedAt `gorm:"index"`
}

func (*User) TableName() string {
	return "user"
}

// 迁移生成表
func AutoMigrate() {
	err := db.Set("gorm:table_options", "ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci COMMENT='用户表'").AutoMigrate(&User{})
	if err != nil {
		fmt.Println("AutoMigrate", err)
	}
}

// Create 创建数据
func Create(u *User) error {
	return db.Create(u).Error
}

// Get 获取单条
// SELECT * FROM `user` WHERE name='laixhe' AND `user`.`deleted_at` IS NULL LIMIT 1
// 没有查找到数据不会报 ErrRecordNotFound 错误
func Get(u *User) error {
	return db.Where("name=?", u.Name).Limit(1).Find(u).Error
}

// GetName 获取单条
// SELECT * FROM `user` WHERE name='laixhe' AND `user`.`deleted_at` IS NULL ORDER BY `user`.`id` LIMIT 1
// 没有查找到数据会报 ErrRecordNotFound 错误
func GetName(u *User) error {
	return db.Where("name=?", u.Name).First(u).Error
}

// Select 获取多条
// SELECT * FROM `user` WHERE `user`.`deleted_at` IS NULL
func Select() ([]User, error) {
	var data []User
	err := db.Find(&data).Error
	if err != nil {
		return nil, err
	}
	return data, err
}

// SelectName 获取多条单例
// SELECT `name` FROM `user` WHERE `user`.`deleted_at` IS NULL
func SelectName() ([]string, error) {
	var data []string
	err := db.Model(&User{}).Select("name").Find(&data).Error
	if err != nil {
		return nil, err
	}
	return data, err
}

// Update 修改单例
// UPDATE `user` SET `age`=?,`updated_at`=? WHERE name=?
func Update(u *User) error {
	return db.Model(u).Where("name=?", u.Name).Update("age", u.Age).Error
}

// Updates 修改多例结构体
// UPDATE `user` SET `name`=?,`age`=? WHERE `id` = ?
func Updates(u *User) error {
	return db.Select("name","age").Updates(u).Error
}

// Save 修改或新增
// 1. 当有 primaryKey 主键时 [如果：没值就新增、有值就修改、有值但表没有查找到就新增(先修改，再查，新增)]
// 2. 当没有主键时 先修改，再查，新增
// 3. 当有 CreatedAt time.Time 要主意了，修改时必须要有值
func Save(u *User) error {
	return db.Where("name=?", u.Name).Save(u).Error
}

// Expr 对某个字段的自增自减
// UPDATE `user` SET `score`=score-1,`updated_at`='2021-02-01 15:39:52.683' WHERE `id` = 10
func Expr(u *User) error {
	return db.Model(u).Update("score", gorm.Expr("score-?", 1)).Error // UpdateColumn
}

// Exprs 对某个字段的自增自减
// UPDATE `user` SET `age`=`age`+2,`score`=`score`+1,`updated_at`='2021-02-01 15:47:16.635' WHERE `id` = 10
func Exprs(u *User) error {
	return db.Model(u).Updates(map[string]interface{}{"score": gorm.Expr("`score`+?", 1), "age": gorm.Expr("`age`+?", 2)}).Error
}

// ScanList 原生 SQL 查询多条结构体
func ScanList() ([]User, error) {
	data := make([]User, 0)
	query := "SELECT `id`,`name`,`age`,`score`,`is` FROM `user`"
	err := db.Raw(query).Scan(&data).Error
	if err != nil {
		return nil, err
	}
	return data, nil
}

// ScanGet 原生 SQL 查询单条结构体
func ScanGet() (*User, error) {
	var data = new(User)
	query := "SELECT `id`,`name`,`age`,`score`,`is` FROM `user` LIMIT 1"
	err := db.Raw(query).Scan(data).Error
	if err != nil {
		return nil, err
	}
	return data, nil
}

// ScanListName 原生 SQL 查询多条单列
func ScanListName() ([]string, error) {
	data := make([]string, 0)
	query := "SELECT `id` FROM `user`"
	err := db.Raw(query).Scan(&data).Error
	if err != nil {
		return nil, err
	}
	return data, nil
}

// ScanGet 原生 SQL 查询单条单列
func ScanGetID() (int, error) {
	var data int
	query := "SELECT `id` FROM `user` LIMIT 1"
	err := db.Raw(query).Scan(&data).Error
	if err != nil {
		return 0, err
	}
	return data, nil
}
