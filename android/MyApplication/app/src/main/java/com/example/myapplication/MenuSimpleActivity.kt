package com.example.myapplication

import android.os.Bundle
import android.view.Menu
import android.view.MenuItem
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity

// Menu 简单使用 - 右上角的
class MenuSimpleActivity: AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_menu_simple)
    }

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