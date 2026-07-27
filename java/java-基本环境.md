#### 集成 Java 环境配置(GraalVM)
```
# Download GraalVM
https://www.graalvm.org/downloads

export GRAALVM_HOME=xxx
export JAVA_HOME=xxx

export MAVEN_HOME=xxx

java --version
mvn -v
```

#### Maven 配置
```
# 打开 maven 的安装目录，选择 conf 文件夹中的 setting.xml 文件
# 修改本地仓库存存放目录：找到 settings.xml 中的 localRepository 配置

# 检验下是否已经设置成功
mvn help:system

# 找到<mirrors></mirrors>标签节点下，配置国内镜像配置
<mirror>
    <id>aliyunmaven</id>
    <mirrorOf>*</mirrorOf>
    <name>阿里云公共仓库</name>
    <url>https://maven.aliyun.com/repository/public</url>
</mirror>
### 或
<mirror>
    <id>huaweicloud</id>
    <mirrorOf>*</mirrorOf>
    <url>https://repo.huaweicloud.com/repository/maven/</url>
</mirror>
### 或
<mirror>
    <id>nexus-163</id>
    <mirrorOf>*</mirrorOf>
    <name>Nexus 163</name>
    <url>http://mirrors.163.com/maven/repository/maven-public/</url>
</mirror>

# 找到<profiles></profiles>标签节点，配置 java 版本
<profile>
  <id>jdk-1.8</id>
  <activation>
    <activeByDefault>true</activeByDefault>
    <jdk>1.8</jdk>
  </activation>
  <properties>
    <maven.compiler.source>1.8</maven.compiler.source>
    <maven.compiler.target>1.8</maven.compiler.target>
    <maven.compiler.compilerVersion>1.8</maven.compiler.compilerVersion>
  </properties>
</profile>
### 或
<profile>
  <id>jdk-17</id>
  <activation>
    <activeByDefault>true</activeByDefault>
    <jdk>17</jdk>
  </activation>
  <properties>
    <maven.compiler.source>17</maven.compiler.source>
    <maven.compiler.target>17</maven.compiler.target>
    <maven.compiler.compilerVersion>17</maven.compiler.compilerVersion>
    <encoding>UTF-8</encoding>
  </properties>
</profile>
```

#### 在项目设置 utf8 编码格式
```
# 在编译的时候通过 project.build.sourceEncoding 属性设置字符编码，通知 maven 使用 UTF-8 编译项目 
<properties>
    <project.build.sourceEncoding>UTF-8</project.build.sourceEncoding>
    <project.reporting.outputEncoding>UTF-8</project.reporting.outputEncoding>
</properties>
```

#### Maven 
```
clean    清理
compile  编译主程序
package  打包
install  安装
```
