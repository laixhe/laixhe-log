package models

import "golang_log/library/api-demo/core/gormx"

// UserTable 表名
const UserTable = "user"

// User 用户表
type User struct {
	Uid      uint64  `gorm:"column:uid;type:int(10) UNSIGNED NOT NULL AUTO_INCREMENT;primaryKey;comment:用户ID自增"`
	Password string  `gorm:"column:password;type:string;size:100;not null;default:'';comment:密码"`
	Email    string  `gorm:"column:email;type:string;size:100;not null;default:'';comment:邮箱"`
	Name     string  `gorm:"column:name;type:string;size:100;not null;default:'';comment:用户名"`
	Age      uint32  `gorm:"column:age;type:tinyint unsigned;not null;default:0;comment:年龄"`
	Score    float64 `gorm:"column:score;type:decimal(5,2) unsigned;not null;default:0.00;comment:分数"`
}

func (*User) TableName() string {
	return UserTable
}

func (*User) Create(user *User) error {
	return gormx.Create(user)
}

func (*User) FirstEmail(email string) (User, error) {
	var user User
	err := gormx.Where("email = ?", email).First(&user).Error
	return user, err
}
