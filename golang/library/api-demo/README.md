### API 接口


##### 生成注释文档
- 在包含 main.go 文件的项目根目录运行 swag init 这将会解析注释并生成需要的文件（docs文件夹和docs/docs.go）
```
swag init
```

##### 给 protobuf 添加结构体 tag
```
protoc-go-inject-tag -input="./api/gen/v1api/*.pb.go"
```
