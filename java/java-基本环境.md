# Java 基本环境

## 为什么用 GraalVM 25

GraalVM 25 是 Oracle 推出的高性能 JDK（基于 Java 25 LTS），除了兼容标准 JDK，还额外支持：

- **原生镜像（Native Image）**：把 Java 程序 AOT 编译成独立可执行文件，启动更快、占用更低。
- **多语言支持**：可运行 JavaScript、Python、Ruby 等语言（GraalVM 20+ 已逐步拆分）。
- **Truffle 框架**：构建自定义语言解释器。

如果只是普通后端开发，用标准 OpenJDK 25 也完全够用；需要极快启动或微服务场景再考虑 GraalVM。

## 安装 JDK / GraalVM 并配置环境变量

下载地址：<https://www.graalvm.org/downloads>（选择 GraalVM for JDK 25；普通 JDK 可选 Adoptium / Microsoft OpenJDK / Oracle OpenJDK 的 25 版本）。

下载解压后，把环境变量写入 `~/.bashrc`（仅当前用户生效）或 `/etc/profile`（所有用户生效）：

```bash
# GraalVM 根目录（普通 JDK 只需配置 JAVA_HOME）
export GRAALVM_HOME=/path/to/graalvm
# JAVA_HOME 指向 GraalVM 或标准 JDK 的安装目录
export JAVA_HOME=/path/to/jdk

# Maven 安装目录
export MAVEN_HOME=/path/to/maven

# 把可执行文件目录加入 PATH
export PATH=$JAVA_HOME/bin:$MAVEN_HOME/bin:$PATH
```

保存后让配置立即生效：

```bash
source ~/.bashrc
```

验证：

```bash
java --version
mvn -v
```

> 说明：`GRAALVM_HOME` 只是 GraalVM 特有的约定，`JAVA_HOME` 才是绝大多数工具（Maven、Gradle、IDE）依赖的关键变量，两者不能混为一谈。

## Maven 配置

Maven 全局配置位于安装目录 `conf/settings.xml`。

### 修改本地仓库目录

找到 `settings.xml` 中的 `localRepository` 节点，改为自己的路径：

```xml
<localRepository>/path/to/your/repo</localRepository>
```

验证是否生效：

```bash
mvn help:system
```

### 配置国内镜像

在 `<mirrors></mirrors>` 节点下添加一个镜像源（任选其一即可）：

```xml
<mirror>
    <id>aliyunmaven</id>
    <mirrorOf>*</mirrorOf>
    <name>阿里云公共仓库</name>
    <url>https://maven.aliyun.com/repository/public</url>
</mirror>
```

或华为云：

```xml
<mirror>
    <id>huaweicloud</id>
    <mirrorOf>*</mirrorOf>
    <url>https://repo.huaweicloud.com/repository/maven/</url>
</mirror>
```

或网易 163：

```xml
<mirror>
    <id>nexus-163</id>
    <mirrorOf>*</mirrorOf>
    <name>Nexus 163</name>
    <url>https://mirrors.163.com/maven/repository/maven-public/</url>
</mirror>
```

### 配置 JDK 版本

在 `<profiles></profiles>` 节点下配置默认编译版本。推荐使用 **25**（LTS），Java 8 已停止免费安全更新：

```xml
<profile>
  <id>jdk-25</id>
  <activation>
    <activeByDefault>true</activeByDefault>
    <jdk>25</jdk>
  </activation>
  <properties>
    <maven.compiler.release>25</maven.compiler.release>
    <encoding>UTF-8</encoding>
  </properties>
</profile>
```

> 现代写法推荐用 `maven.compiler.release`（一次设定即可），旧写法需同时设 `maven.compiler.source`、`maven.compiler.target`、`maven.compiler.compilerVersion` 三个属性。

## 在项目设置 UTF-8 编码

在项目的 `pom.xml` 中加入以下属性，通知 Maven 使用 UTF-8 编译：

```xml
<properties>
    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    <project.reporting.outputEncoding>UTF-8</project.reporting.outputEncoding>
</properties>
```

## 常用 Maven 命令

在项目目录（含 `pom.xml`）下执行，均以 `mvn` 开头：

```bash
mvn clean      # 清理 target 目录中的构建产物
mvn compile    # 编译主程序源码到 target/classes
mvn test       # 编译并运行单元测试
mvn package    # 编译 + 测试 + 打包（jar/war）
mvn install    # 打包后安装到本地仓库，供其他项目依赖
mvn deploy     # 打包后发布到远程仓库
```

常用参数：

```bash
mvn package -DskipTests      # 打包时跳过测试
mvn package -T 4             # 使用 4 个线程并行构建
mvn clean package            # 先清理再打包
```
