package com.laixhe.springbootdemo.interceptor;

import lombok.extern.slf4j.Slf4j;
import org.springframework.web.servlet.HandlerInterceptor;

import javax.servlet.http.HttpServletRequest;
import javax.servlet.http.HttpServletResponse;

// 登录拦截器
@Slf4j
public class LoginInterceptor implements HandlerInterceptor {

    // 执行前
    @Override
    public boolean preHandle(HttpServletRequest request, HttpServletResponse response, Object handler) throws Exception {
        // 登录检查逻辑
        // ...
        // ...

        var authorization = request.getHeader("Authorization");
        log.info("authorization:"+authorization);

        if(authorization==null) {
            return false;
        }
        return authorization.length() != 0;
    }
}
