package com.example.myapplication

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityFragmentSimpleBinding

// Fragment 简单应用
class FragmentSimpleActivity: AppCompatActivity() {

    private val mainFragment = FragmentSimpleMain() // 首页 Fragment
    private val findFragment = FragmentSimpleFind() // 发现 Fragment
    private val meFragment = FragmentSimpleMe()     // 我的 Fragment

    // 视图绑定
    private lateinit var binding: ActivityFragmentSimpleBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityFragmentSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)
        // 获取 Fragment 管理类
        val fragmentManager = supportFragmentManager
        // 开始事务
        fragmentManager.beginTransaction()
            .add(binding.fragmentSimpleContent.id, mainFragment)                    // 添加并显示首页
            .add(binding.fragmentSimpleContent.id, findFragment).hide(findFragment) // 添加并隐藏发现
            .add(binding.fragmentSimpleContent.id, meFragment).hide(meFragment)     // 添加并隐藏我的
            .commit() // 提交事务

        // 初始化选中状态
        binding.fragmentSimpleMenuMain.isSelected = true
        binding.fragmentSimpleMenuFind.isSelected = false
        binding.fragmentSimpleMenuMe.isSelected = false

        // 点击事件
        menuClick()
    }

    // 全部选中状态
    private fun setAllFalse(){
        binding.fragmentSimpleMenuMain.isSelected = false
        binding.fragmentSimpleMenuFind.isSelected = false
        binding.fragmentSimpleMenuMe.isSelected = false
    }

    // 点击事件
    private fun menuClick(){
        // 首页
        binding.fragmentSimpleMenuMain.setOnClickListener {
            setAllFalse()
            binding.fragmentSimpleMenuMain.isSelected = true

            // 获取 Fragment 管理类
            val fragmentManager = supportFragmentManager
            // 开始事务
            fragmentManager.beginTransaction()
                .show(mainFragment)
                .hide(findFragment)
                .hide(meFragment)
                .commit() // 提交事务
        }

        // 发现
        binding.fragmentSimpleMenuFind.setOnClickListener {
            setAllFalse()
            binding.fragmentSimpleMenuFind.isSelected = true

            // 获取 Fragment 管理类
            val fragmentManager = supportFragmentManager
            // 开始事务
            fragmentManager.beginTransaction()
                .hide(mainFragment)
                .show(findFragment)
                .hide(meFragment)
                .commit() // 提交事务
        }

        // 我的
        binding.fragmentSimpleMenuMe.setOnClickListener {
            setAllFalse()
            binding.fragmentSimpleMenuMe.isSelected = true

            // 获取 Fragment 管理类
            val fragmentManager = supportFragmentManager
            // 开始事务
            fragmentManager.beginTransaction()
                .hide(mainFragment)
                .hide(findFragment)
                .show(meFragment)
                .commit() // 提交事务
        }
    }

}