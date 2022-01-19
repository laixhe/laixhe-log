package com.example.myapplication

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityButtonSimpleBinding
import com.google.android.material.snackbar.Snackbar

// Button 按钮
class ButtonSimpleActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityButtonSimpleBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivityButtonSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        binding.buttonSimpleFab.setOnClickListener {

            Snackbar.make(it, "虚浮按钮", Snackbar.LENGTH_SHORT)
                .setAction("确定") {
                    Toast.makeText(this, "你点了确定...", Toast.LENGTH_LONG).show()
                }.show()
        }
    }

}