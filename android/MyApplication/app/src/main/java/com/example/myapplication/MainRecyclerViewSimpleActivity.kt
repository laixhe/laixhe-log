package com.example.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.util.Log
import androidx.recyclerview.widget.GridLayoutManager
import androidx.recyclerview.widget.LinearLayoutManager
import androidx.recyclerview.widget.StaggeredGridLayoutManager
import com.example.myapplication.databinding.ActivityMainRecyclerViewSimpleBinding

// RecyclerView 简单
class MainRecyclerViewSimpleActivity : AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityMainRecyclerViewSimpleBinding

    // 数据
    private var data: MutableList<MainRecyclerViewSimpleData> = mutableListOf<MainRecyclerViewSimpleData>()

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityMainRecyclerViewSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        for(i in 0..99){
            data.add(MainRecyclerViewSimpleData("test-r-$i"))
        }

        // 先设置 recycler 的布局管理 (如：线性布局)
        binding.recyclerViewSimple.layoutManager = LinearLayoutManager(this)
        // 先设置 recycler 的布局管理 (如：瀑布流布局)
        //binding.recyclerViewSimple.layoutManager = StaggeredGridLayoutManager(3, StaggeredGridLayoutManager.VERTICAL)
        // 先设置 recycler 的布局管理 (如：网格布局)
        //binding.recyclerViewSimple.layoutManager = GridLayoutManager(this, 3)
        // 设置 recycler view adapter
        val recyclerViewSimpleAdapter =  MainRecyclerViewSimpleAdapter(data, this)
        binding.recyclerViewSimple.adapter = recyclerViewSimpleAdapter

        // 由于 RecyclerView 没有点击事件
        // 自己实现回调点击事件
        recyclerViewSimpleAdapter.setRecyclerItemClickListener(object: MainRecyclerViewSimpleAdapter.OnRecyclerItemClickListener{
            override fun onRecyclerItemClick(position: Int) {
                Log.e("T--------", "onRecyclerItemClick=$position")
            }
        })

    }
}