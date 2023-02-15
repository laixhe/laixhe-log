
#### 支持多种数据库 mysql postgresql oracle sqlite

> 安装
```
go get github.com/jmoiron/sqlx
```

> 导入
```
import "github.com/jmoiron/sqlx"
```

> 查询，sqlx.DB.Get 和 sqlx.DB.Select

> 更新、插入、删除，sqlx.DB.Exec

> 事务，sqlx.DB.Begin() 、sqlx.DB.Commit() 、sqlx.DB.Rollback()

> 与原生 database/sql 相识
 