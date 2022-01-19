package com.laixhe.springbootdemo.controller;

import com.laixhe.springbootdemo.bean.DbConfig;
import com.laixhe.springbootdemo.entity.model.User;
import com.laixhe.springbootdemo.service.impl.UserServiceImpl;
import org.springframework.web.bind.annotation.*;
import org.springframework.web.multipart.MultipartFile;

import java.io.IOException;
import java.nio.file.Path;
import java.util.HashMap;
import java.util.List;
import java.util.Map;

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
    final UserServiceImpl userService;
    public IndexController(DbConfig dbConfig, UserServiceImpl userService) {
        this.dbConfig = dbConfig;
        this.userService = userService;
    }

    @GetMapping("/hello")
    public String hello(){
        // http://localhost:8080/hello

        return "Hello World...";
    }

    @GetMapping("/get-db-config")
    public DbConfig getDbConfig(){
        // http://localhost:8080/get-db-config

        return dbConfig;
    }

    @GetMapping("/test/{id}")
    public Map<String, Object> getTest(
            @PathVariable("id") Integer id,               // 获取路径参数 (key 区分大小写)
            @RequestParam("name") String name,            // 获取 GET 参数 (key 区分大小写)
            @RequestParam("likes") List<String> likes,    // 获取 GET 参数
            @RequestHeader("user-agent") String userAgent,// 获取头信息 (key 不区分大小写)
            @CookieValue("age") Integer age               // 获取 Cookie (key 区分大小写)
    ){

        // http://localhost:8080/test/123?name=laixhe&likes=%E7%88%AC%E5%B1%B1&likes=%E9%AA%91%E8%BD%A6
        // http://localhost:8080/test/123?name=laixhe&likes=爬山&likes=骑车

        Map<String, Object> map = new HashMap<>();

        map.put("id", id);
        map.put("name", name);
        map.put("age", age);
        map.put("likes", likes);
        map.put("user-agent", userAgent);

        return map;
    }

    @PostMapping("/test/post")
    public Map<String, Object> postTest(
            @RequestParam("id") Integer id,     // 获取 Post 参数 (key 区分大小写)
            @RequestParam("name") String name   // 获取 Post 参数
    ){

        // http://localhost:8080/test/post
        // id=19&name=lai

        Map<String, Object> map = new HashMap<>();

        map.put("id", id);
        map.put("name", name);

        return map;
    }

    @PostMapping("/test/file")
    public Map<String, Object> postFile(
            @RequestParam("ufile") MultipartFile uFile // 文件上传
    ){
        Map<String, Object> map = new HashMap<>();

        map.put("file_name", uFile.getOriginalFilename());
        map.put("file_size", uFile.getSize());
        map.put("file_type", uFile.getContentType());

        if(!uFile.isEmpty()) {
            // 保存文件
            try {
                uFile.transferTo(Path.of("./__"+uFile.getOriginalFilename()));
            } catch (IOException e) {
                e.printStackTrace();
            }
        }

        return map;
    }

    @GetMapping("/db/user")
    public User getById(@RequestParam("id") Long id){
        return userService.getById(id);
    }

}
