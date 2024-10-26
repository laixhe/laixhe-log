
### client-go 提供了四种客户端
- RESTClient: 这是最基础的客户端对象，仅对 HTTP Request 进行了封装，实现 RESTFul 风格 API，这个对象的使用并不方便，因为很多参数都要使用者来设置
- ClientSet: 是对 RESTClient 做了一层封装，把每一种资源都封装了对应的客户端。把 Resource 和 Version 也封装成方法了，用起来更简单直接，一个资源是一个客户端，多个资源就对应了多个客户端，所以 ClientSet 就是多个客户端的集合，不过 ClientSet 只能访问内置资源，访问不了自定义资源
- DynamicClient: 可以访问内置资源和自定义资源，拿出的内容是 Object 类型，按实际情况自己去做强制转换，当然了也会有强转失败的风险
- DiscoveryClient: 用于发现 kubernetes 的 API Server 支持的 Group、Version、Resources 等信息

### 编写文件
- pkg/apis/crdtest/register.go
- pkg/apis/crdtest/v1/register.go
- pkg/apis/crdtest/v1/doc.go
- pkg/apis/crdtest/v1/databasemanager_types.go

### 代码生成工具 code-generator
- deepcopy-gen：生成深度拷贝方法，为每个 T 类型生成 func (t* T) DeepCopy() *T 方法，API 类型都需要实现深拷贝
- client-gen：为资源生成标准的 clientset
- informer-gen：生成 informer，提供事件机制来响应资源的事件
- lister-gen：生成 Lister**，**为 get 和 list 请求提供只读缓存层（通过 indexer 获取）

```
go install k8s.io/code-generator/cmd/{defaulter-gen,client-gen,lister-gen,informer-gen,deepcopy-gen}
```

### 生成代码
```
go mod vendor
chmod +x ./hack/update-codegen.sh
chmod +x ./vendor/k8s.io/code-generator/generate-groups.sh
./hack/update-codegen.sh
```

### 生成 crd yaml 文件
```
git clone https://github.com/kubernetes-sigs/controller-tools.git
cd controller-tools
go install ./cmd/{controller-gen,type-scaffold}

# 生成 crd yaml 文件
controller-gen crd paths=./... output:crd:dir=artifacts
```

### 导入
```
kubectl apply -f artifacts/crd-database-manager.yaml

kubectl apply -f artifacts/mysql-database-manager.yaml
kubectl delete -f artifacts/mysql-database-manager.yaml
```

### 启动
```
./goclient -kubeconfig=$HOME/.kube/config -alsologtostderr=true
```

###
- 自定义 Controller 上 https://developer.aliyun.com/article/943389
- 自定义 Controller 下 https://developer.aliyun.com/article/943395
