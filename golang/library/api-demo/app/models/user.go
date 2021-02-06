package models

// User 用户表
type User struct {
	Id    int     `gorm:"column:id;type:int(10) UNSIGNED NOT NULL AUTO_INCREMENT;primaryKey;comment:用户ID自增"`
	Name  string  `gorm:"column:name;type:string;size:100;not null;default:'';comment:用户名"`
	Age   int     `gorm:"column:age;type:tinyint unsigned;not null;default:0;comment:年龄"`
	Score float64 `gorm:"column:score;type:decimal(5,2) unsigned;not null;default:0.00;comment:分数"`
}
