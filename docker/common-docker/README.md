# common-docker

一套开箱即用的多服务本地开发编排，覆盖常用中间件：etcd、Redis、MongoDB、Kafka、PostgreSQL、MySQL、MinIO、Meilisearch、Zookeeper。

## 前置条件

- 已安装 Docker（推荐 Docker Desktop 或 docker-ce）。
- 已安装 Docker Compose v2（`docker compose version` 可查看，命令中间是空格）。

## 启动

在 `docker-compose.yml` 同目录下执行：

```bash
# 启动全部服务（后台运行）
docker compose up -d

# 只启动单个服务，例如 Kafka
docker compose up -d kafka

# 查看运行状态
docker compose ps

# 查看某个服务日志（实时滚动）
docker compose logs -f mysql
```

> 提示：
> - **首次启动会自动拉取镜像**，视网络情况可能需要几分钟，属正常现象。
> - 服务「就绪」需要一点时间（如 MySQL 首次启动要先初始化数据目录）。可用 `docker compose ps` 看状态，或 `docker compose logs -f mysql` 看日志确认。
> - `-d` 表示后台运行；去掉 `-d` 会前台运行并占用当前终端。

## 服务清单

| 服务 | 端口 | 账号 / 密码 | 说明 |
| --- | --- | --- | --- |
| etcd | 2379（客户端）/ 2380（peer） | 无认证 | 分布式键值存储 |
| redis | 6379 | 无密码 | 缓存，开启 AOF 持久化 |
| mongo | 27017 | `admin` / `123456` | 开启 `--auth`，需账号密码登录 |
| kafka | 9092（对外）/ 9093（controller） | 无认证 | 单节点 broker+controller |
| postgresql | 5432 | `postgres` / `123456`，库 `postgres` | 关系型数据库 |
| mysql | 3306 | `root` / `123456` | 允许任意 host 登录 root |
| minio | 9000（API）/ 9090（控制台） | `root` / `12345678` | 对象存储 |
| meilisearch | 7700 | 无主密钥（development 模式） | 搜索引擎 |
| zookeeper | 2181（客户端）/ 8080（管理） | 匿名登录 | 分布式协调 |

> 账号密码在 [docker-compose.yml](./docker-compose.yml) 的 `environment` 中配置，仅用于本地开发，请勿在生产环境沿用默认密码。

## 镜像版本

各服务镜像版本统一在 [.env](./.env) 中维护，修改后重启即可升级：

```bash
# 修改 .env 后重建容器（tag 变了会自动拉取新镜像）
docker compose up -d

# 即使 tag 没变也强制重新拉取并重建（例如远程镜像更新了）
docker compose up -d --pull always
```

## 停止与清理

> 各服务的数据已持久化到 Docker 命名卷（`docker volume ls` 可查看），所以：
>
> - `docker compose down` 只停容器，**数据保留**
> - `docker compose down -v` 会连数据卷一起删除，**数据清空**（慎用）

```bash
# 停止所有服务（保留数据）
docker compose down

# 停止并删除数据卷、网络（慎用，会清空容器数据）
docker compose down -v

# 数据卷存在哪里？可以通过以下命令查看/清理
docker volume ls
```

## 访问示例

- MySQL：`mysql -h 127.0.0.1 -P 3306 -uroot -p123456`
- Redis：`redis-cli -h 127.0.0.1 -p 6379`
- MinIO 控制台：浏览器打开 `http://localhost:9090`
- Meilisearch：浏览器打开 `http://localhost:7700`

> 上面的 `mysql` / `redis-cli` 需要本机装有对应客户端；没装的话，可以直接进容器里用：
>
> ```bash
> docker exec -it mysql mysql -uroot -p123456
> docker exec -it redis redis-cli
> ```
