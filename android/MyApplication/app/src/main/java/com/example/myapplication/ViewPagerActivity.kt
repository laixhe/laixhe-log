package com.example.myapplication

import android.os.Bundle
import android.view.View
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityViewPager1Binding
import com.example.myapplication.databinding.ActivityViewPager2Binding
import com.example.myapplication.databinding.ActivityViewPager3Binding
import com.example.myapplication.databinding.ActivityViewPagerBinding

// ViewPager 多页面切换 1
class ViewPagerActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityViewPagerBinding

    private lateinit var bindingView1: ActivityViewPager1Binding
    private lateinit var bindingView2: ActivityViewPager2Binding
    private lateinit var bindingView3: ActivityViewPager3Binding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityViewPagerBinding.inflate(layoutInflater)
        setContentView(binding.root)

        bindingView1 = ActivityViewPager1Binding.inflate(layoutInflater)
        bindingView2 = ActivityViewPager2Binding.inflate(layoutInflater)
        bindingView3 = ActivityViewPager3Binding.inflate(layoutInflater)

        val viewList = arrayListOf<View>(bindingView1.root, bindingView2.root, bindingView3.root)

        binding.viewPagerVp.adapter = ViewPagerAdapter(viewList)

    }
}