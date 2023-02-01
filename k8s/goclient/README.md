
### client-go 提供了四种客户端
- RESTClient: 这是最基础的客户端对象，仅对 HTTP Request 进行了封装，实现 RESTFul 风格 API，这个对象的使用并不方便，因为很多参数都要使用者来设置
- ClientSet: 是对 RESTClient 做了一层封装，把每一种资源都封装了对应的客户端。把 Resource 和 Version 也封装成方法了，用起来更简单直接，一个资源是一个客户端，多个资源就对应了多个客户端，所以 ClientSet 就是多个客户端的集合，不过 ClientSet 只能访问内置资源，访问不了自定义资源
- DynamicClient: 可以访问内置资源和自定义资源，拿出的内容是 Object 类型，按实际情况自己去做强制转换，当然了也会有强转失败的风险
- DiscoveryClient: 用于发现 kubernetes 的 API Server 支持的 Group、Version、Resources 等信息

