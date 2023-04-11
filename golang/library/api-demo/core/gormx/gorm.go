package gormx

import (
	"time"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
	"gorm.io/gorm/schema"

	"golang_log/library/api-demo/core/logx"
)

type Config struct {
	LogMode      Mode   // 运行模式 debug release
	Dsn          string // 连接地址
	MaxIdleCount int    // 设置空闲连接池中连接的最大数量
	MaxOpenCount int    // 设置打开数据库连接的最大数量
	MaxLifeTime  int    // 设置了连接可复用的最大时间(要比数据库设置连接超时时间少)(单位秒)
}

var db *gorm.DB

// Ping 测试
func Ping() error {
	sqlDB, err := db.DB()
	if err != nil {
		return err
	}
	// 验证数据库连接是否正常
	err = sqlDB.Ping()
	if err != nil {
		return err
	}
	return nil
}

// DB get db client
func DB() *gorm.DB {
	return db
}

// Init 初始化数据库
func Init(c *Config) {
	logx.Debug("db 开始初始化...")

	if c.MaxIdleCount < 0 {
		c.MaxIdleCount = 0
	}
	if c.MaxOpenCount < 0 {
		c.MaxOpenCount = 0
	}
	if c.MaxLifeTime < 0 {
		c.MaxLifeTime = 0
	}
	var logMode = logger.Silent
	if c.LogMode == ModeDebug {
		logMode = logger.Info
	}

	var err error
	db, err = gorm.Open(mysql.Open(c.Dsn), &gorm.Config{
		Logger: logger.Default.LogMode(logMode),
		NamingStrategy: schema.NamingStrategy{
			SingularTable: true, // 使用单数表名，启用该选项后，`User` 表将是`user`
		},
	})
	if err != nil {
		panic(err)
	}

	sqlDB, err := db.DB()
	if err != nil {
		panic(err)
	}
	if c.MaxIdleCount > 0 {
		// 设置空闲连接池中连接的最大数量
		sqlDB.SetMaxIdleConns(c.MaxIdleCount)
	}
	if c.MaxOpenCount > 0 {
		// 设置打开数据库连接的最大数量
		sqlDB.SetMaxOpenConns(c.MaxOpenCount)
	}
	if c.MaxLifeTime > 0 {
		// 设置了连接可复用的最大时间(要比数据库设置连接超时时间少)
		sqlDB.SetConnMaxLifetime(time.Duration(c.MaxLifeTime) * time.Second)
	}
	// 验证数据库连接是否正常
	err = sqlDB.Ping()
	if err != nil {
		panic(err)
	}

	logx.Debug("db 初始化完毕...")
}
