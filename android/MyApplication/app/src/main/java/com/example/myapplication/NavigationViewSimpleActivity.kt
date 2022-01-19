package com.example.myapplication

import android.os.Bundle
import android.view.MenuItem
import androidx.appcompat.app.AppCompatActivity
import androidx.core.view.GravityCompat
import com.example.myapplication.databinding.ActivityNavigationViewSimpleBinding
import com.google.android.material.navigation.NavigationView

// NavigationView 侧滑菜单-简单使用
class NavigationViewSimpleActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityNavigationViewSimpleBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivityNavigationViewSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // 设置标题栏
        setSupportActionBar(binding.navigationViewSimpleToolbar)

        // 返回按钮点击事件
        binding.navigationViewSimpleToolbar.setNavigationOnClickListener {
            // 弹出 侧滑菜单
            binding.navigationViewSimpleId.openDrawer(GravityCompat.START)
        }

        // 默认选中的
        binding.navigationViewSimpleNavigationView.setCheckedItem(R.id.menu_navigation_view_simple_call)
        binding.navigationViewSimpleNavigationView.setNavigationItemSelectedListener(object : NavigationView.OnNavigationItemSelectedListener{
            override fun onNavigationItemSelected(item: MenuItem): Boolean {
                // 关闭 侧滑菜单
                binding.navigationViewSimpleId.closeDrawers()
                return true
            }
        })
    }
}