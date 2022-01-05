package com.laixhe.springbootdemo.config;

import com.laixhe.springbootdemo.interceptor.LoginInterceptor;
import org.springframework.context.annotation.Configuration;
import org.springframework.web.servlet.config.annotation.InterceptorRegistry;
import org.springframework.web.servlet.config.annotation.WebMvcConfigurer;

// 拦截器配置
@Configuration
public class AuthorizationConfig implements WebMvcConfigurer {
    // 添加拦截器的容器
//    @Override
//    public void addInterceptors(InterceptorRegistry registry) {
//        registry.addInterceptor( new LoginInterceptor())           // 添加拦截器
//                .addPathPatterns("/**")                            // 拦截所有请求
//                .excludePathPatterns("/db/user");  // 对那些请求不进行拦截
//    }
}
