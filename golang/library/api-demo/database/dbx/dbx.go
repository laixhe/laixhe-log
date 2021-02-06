package dbx

import (
	"golang_log/library/api-demo/config"

	"github.com/laixhe/goutil/zaplog"

	"gorm.io/driver/mysql"
	"gorm.io/gorm"
	"gorm.io/gorm/logger"
)

var (
	db *gorm.DB
)

// 初始化数据库
func init() {
	zaplog.Debug("db 开始初始化...")

	var logMode = logger.Silent
	if config.IsAppDebug() {
		logMode = logger.Info
	}

	var err error
	db, err = gorm.Open(mysql.Open(config.DBDsn()), &gorm.Config{
		Logger: logger.Default.LogMode(logMode),
	})
	if err != nil {
		panic(err)
	}

	sqlDB, err := db.DB()
	if err != nil {
		panic(err)
	}

	// 设置空闲连接池中连接的最大数量
	sqlDB.SetMaxIdleConns(config.DBMaxIdleConn())

	// 设置打开数据库连接的最大数量
	sqlDB.SetMaxOpenConns(config.DBMaxOpenConn())

	// 验证数据库连接是否正常
	err = sqlDB.Ping()
	if err != nil {
		panic(err)
	}

	zaplog.Debug("db 初始化完毕...")
}

// Ping 测试
func Ping() error {
	sqlDB, err := db.DB()
	if err != nil {
		zaplog.Errorf("db ping error:%v", err)
		return err
	}

	// 验证数据库连接是否正常
	err = sqlDB.Ping()
	if err != nil {
		zaplog.Errorf("db ping error:%v", err)
		return err
	}

	return nil
}

// DB get db
func DB() *gorm.DB {
	return db
}
