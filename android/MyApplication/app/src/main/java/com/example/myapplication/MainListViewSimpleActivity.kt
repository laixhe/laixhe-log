package com.example.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import com.example.myapplication.databinding.ActivityMainListViewSimpleBinding

// ListView 简单
class MainListViewSimpleActivity : AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityMainListViewSimpleBinding

    // 数据
    private var data: MutableList<MainListViewSimpleData> = mutableListOf<MainListViewSimpleData>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityMainListViewSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        for(i in 0..99){
            data.add(MainListViewSimpleData("test-l-$i"))
        }

        // 设置 list view adapter
        binding.listViewSimple.adapter = MainListViewSimpleAdapter(data, this)
        // 点击事件
        binding.listViewSimple.setOnItemClickListener { parent, view, position, id ->
            Log.e("T------", "OnItemClickListener $position")
        }
    }
}