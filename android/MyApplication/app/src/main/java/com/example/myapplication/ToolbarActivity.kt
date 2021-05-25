package com.example.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import androidx.appcompat.widget.Toolbar

// Toolbar 操作(标题)栏
class ToolbarActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_toolbar)

        val toolbarSimple = findViewById<Toolbar>(R.id.toolbar_simple)
        // 返回按钮点击事件
        toolbarSimple.setNavigationOnClickListener {
            // 移除当前页面
            finish()
        }

        val toolbarSimpleCode = findViewById<Toolbar>(R.id.toolbar_simple_code)
        toolbarSimpleCode.title = "标题"
    }
}