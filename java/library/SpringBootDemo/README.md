#### 相关注解
```
# 运行这个类的 main 方法来启动 SpringBoot 应用
@SpringBootApplication
# 配置类
@Configuration (@Configuration + @Bean)
# 给容器中添加组件 (函数)
@Bean
# 组件类
@Component
# 配置绑定(绑定自定义配置 application.properties @Component + @ConfigurationProperties)
@ConfigurationProperties


# 指定扫描路径
@ComponentScan

```

#### 将项目打成 jar 包
- 点击 maven 的 Lifecycle/package 等待生成
- java -jar xxx.jar
