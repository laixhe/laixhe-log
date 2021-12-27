package com.example.myapplication

import android.os.Bundle
import android.view.View
import android.view.ViewGroup
import android.widget.PopupWindow
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityPopupWindowBinding

// PopupWindow 悬浮框
class PopupWindowActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityPopupWindowBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityPopupWindowBinding.inflate(layoutInflater)
        setContentView(binding.root)
    }

    // 弹出 PopupWindow 的 点击事件
    fun popClick(view: View){
        // 获取布局文件
        val popupView = layoutInflater.inflate(R.layout.activity_popup_window_view, null)
        // 创建 PopupWindow
        val popupWindow = PopupWindow(popupView,
            ViewGroup.LayoutParams.WRAP_CONTENT,
            ViewGroup.LayoutParams.WRAP_CONTENT,
            true
        )

        // 显示在那个元素下方
        popupWindow.showAsDropDown(view)
        // 关闭 PopupWindow
        //popupWindow.dismiss()
    }
}