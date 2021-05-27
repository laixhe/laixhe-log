package com.example.myapplication

import android.os.Bundle
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityDrawerLayoutSimpleBinding

// DrawerLayout 侧滑菜单-简单使用
class DrawerLayoutSimpleActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityDrawerLayoutSimpleBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivityDrawerLayoutSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)
    }

}