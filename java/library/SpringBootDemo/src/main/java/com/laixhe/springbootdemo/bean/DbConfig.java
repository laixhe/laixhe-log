package com.laixhe.springbootdemo.bean;

import lombok.Data;
import lombok.ToString;
import org.springframework.boot.context.properties.ConfigurationProperties;
import org.springframework.stereotype.Component;

@Data     // 自动生成 get set
@ToString // 自动生成 toString
@Component
@ConfigurationProperties(prefix = "dbconfig") // 绑定自定义配置 application.properties
public class DbConfig {
    private String name;
    private String dns;
}
