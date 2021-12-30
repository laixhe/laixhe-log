package com.laixhe.springbootdemo.controller;

import com.laixhe.springbootdemo.bean.DbConfig;
import org.springframework.beans.factory.annotation.Autowired;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;

//@Controller
//@ResponseBody
// 或
//@RestController

@RestController
public class IndexController {

    // 自动注入
    //@Autowired
    //DbConfig dbConfig;
    // 或 下面好像是新版本
    //final DbConfig dbConfig;
    //public IndexController(DbConfig dbConfig) {
    //    this.dbConfig = dbConfig;
    //}

    final DbConfig dbConfig;
    public IndexController(DbConfig dbConfig) {
        this.dbConfig = dbConfig;
    }

    @RequestMapping("/")
    public String index(){
        return "说明这是一个 Spring Boot 应用...";
    }

    @RequestMapping("/hello")
    public String hello(){
        return "Hello World...";
    }

    @RequestMapping("/get-db-config")
    public DbConfig getDbConfig(){
        return dbConfig;
    }

}
