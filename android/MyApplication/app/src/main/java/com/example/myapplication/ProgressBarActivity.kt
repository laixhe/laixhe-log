package com.example.myapplication

import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import com.example.myapplication.databinding.ActivityProgressBarBinding

// ProgressBar 进度条
class ProgressBarActivity : AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityProgressBarBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivityProgressBarBinding.inflate(layoutInflater)
        setContentView(binding.root)

    }

    // 默认是转圈
    // 显示隐藏进度条 的 点击事件
    fun leoClick(view: View) {
        if (binding.pbDefault.visibility == View.GONE) {
            // 显示
            binding.pbDefault.visibility = View.VISIBLE
        }else{
            // 隐藏
            binding.pbDefault.visibility = View.GONE
        }
    }

    // 水平(横)进度条 的 点击事件
    fun horizontalClick(view: View) {
        // 获取当前进度的值
        var i = binding.pbHorizontal.progress
        i += 10
        binding.pbHorizontal.progress = i
    }
}