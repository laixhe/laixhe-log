package com.laixhe.springbootdemo.config;

import com.laixhe.springbootdemo.bean.UserConfig;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;

@Configuration // 配置类 - 组件都是单例(默认)
public class MyConfig {

    @Bean // 给容器中添加组件，以方法名作为组件的ID
    public UserConfig userConfig(){
        return new UserConfig("laixhe", 18);
    }
}
