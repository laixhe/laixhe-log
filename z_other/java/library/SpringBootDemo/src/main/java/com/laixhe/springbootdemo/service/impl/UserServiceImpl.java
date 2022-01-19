package com.laixhe.springbootdemo.service.impl;

import com.baomidou.mybatisplus.extension.service.impl.ServiceImpl;
import com.laixhe.springbootdemo.dao.UserMapper;
import com.laixhe.springbootdemo.entity.model.User;
import com.laixhe.springbootdemo.service.UserService;
import org.springframework.stereotype.Service;

@Service
public class UserServiceImpl extends ServiceImpl<UserMapper, User> implements UserService {

    final UserMapper userMapper;

    public UserServiceImpl(UserMapper userMapper) {
        this.userMapper = userMapper;
    }

    public User getUser(Long id){
        return userMapper.selectById(id);
    }
}
