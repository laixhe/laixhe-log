package com.example.myapplication

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.widget.Button

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 加载布局文件
        setContentView(R.layout.activity_main)

        // ListView简单
        // 获取布局的按钮
        val mainListViewSimple = findViewById<Button>(R.id.main_list_view_simple)
        // 点击事件
        mainListViewSimple.setOnClickListener {
            // 跳转页面
            val intent = Intent(this, MainListViewSimpleActivity::class.java)
            startActivity(intent)
        }

        // RecyclerView 简单
        val mainRecyclerViewSimple = findViewById<Button>(R.id.main_recycler_view_simple)
        mainRecyclerViewSimple.setOnClickListener {
            val intent = Intent(this, MainRecyclerViewSimpleActivity::class.java)
            startActivity(intent)
        }

    }
}