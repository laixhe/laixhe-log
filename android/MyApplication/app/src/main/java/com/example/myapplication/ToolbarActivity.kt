package com.example.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import android.widget.Toast
import androidx.appcompat.widget.Toolbar

// Toolbar 操作(标题)栏
class ToolbarActivity : AppCompatActivity() {

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_toolbar)

        val toolbarSimple = findViewById<Toolbar>(R.id.toolbar_simple)
        // 设置标题栏
        setSupportActionBar(toolbarSimple)

        // 返回按钮点击事件
        toolbarSimple.setNavigationOnClickListener {
            // 移除当前页面
            finish()
        }
    }

    // 创建菜单
    override fun onCreateOptionsMenu(menu: Menu?): Boolean {
        menuInflater.inflate(R.menu.menu_simple, menu)
        return true
    }

    override fun onOptionsItemSelected(item: MenuItem): Boolean {
        when(item.itemId){
            R.id.menu_simple_add_item -> {
                Toast.makeText(this, "添加...", Toast.LENGTH_SHORT).show()
            }
            R.id.menu_simple_remove_item -> {
                Toast.makeText(this, "移除...", Toast.LENGTH_SHORT).show()
            }
        }
        return true
    }

}