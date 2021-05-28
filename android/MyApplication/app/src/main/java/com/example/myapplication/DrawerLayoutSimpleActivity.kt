package com.example.myapplication

import android.os.Bundle
import android.view.Gravity
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.GravityCompat
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

        // 设置标题栏
        setSupportActionBar(binding.drawerLayoutSimpleToolbar)

        // 返回按钮点击事件
        binding.drawerLayoutSimpleToolbar.setNavigationOnClickListener {
            // 弹出 侧滑菜单
            binding.drawerLayoutSimpleId.openDrawer(GravityCompat.START)
        }

//        val sActionBar = supportActionBar
//        sActionBar?.setDisplayHomeAsUpEnabled(true)
//        sActionBar?.setHomeAsUpIndicator(R.drawable.ic_fushi)
    }

}