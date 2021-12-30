package com.laixhe.springbootdemo.bean;

import lombok.AllArgsConstructor;
import lombok.Data;
import lombok.NoArgsConstructor;
import lombok.ToString;

@Data               // 自动生成 get set
@ToString           // 自动生成 toString
@AllArgsConstructor // 自动生成 所有参数的构造函数
@NoArgsConstructor  // 自动生成 无参构造函数
public class UserConfig {
    private String name;
    private Integer age;
}
