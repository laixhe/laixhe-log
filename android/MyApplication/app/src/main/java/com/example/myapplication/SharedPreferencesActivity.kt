package com.example.myapplication

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivitySharedPreferencesBinding

// SharedPreferences 存储
// 轻量级的存储类
// 文件存放在 /data/data/xxx/shared_prefs 目录下
class SharedPreferencesActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivitySharedPreferencesBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivitySharedPreferencesBinding.inflate(layoutInflater)
        setContentView(binding.root)

        binding.sharedPreferencesSet.setOnClickListener {

            val sp = getSharedPreferences("data_config", MODE_PRIVATE).edit()
            sp.putString("name", binding.sharedPreferences.text.toString())
            sp.apply()

            Toast.makeText(this, "存入成功", Toast.LENGTH_LONG).show()
        }

        binding.sharedPreferencesGet.setOnClickListener {
            val sp = getSharedPreferences("data_config", MODE_PRIVATE)
            val name = sp.getString("name", "")

            binding.sharedPreferences.setText(name)

            Toast.makeText(this, "获取：$name", Toast.LENGTH_LONG).show()
        }
    }
}