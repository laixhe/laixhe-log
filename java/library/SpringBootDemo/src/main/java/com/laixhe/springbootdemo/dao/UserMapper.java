package com.laixhe.springbootdemo.dao;

import com.baomidou.mybatisplus.core.mapper.BaseMapper;
import com.laixhe.springbootdemo.entity.model.User;
import org.apache.ibatis.annotations.Mapper;

@Mapper
public interface UserMapper extends BaseMapper<User> {
}
