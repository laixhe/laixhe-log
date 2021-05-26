package com.example.myapplication

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import android.widget.Button

class MainActivity : AppCompatActivity() {
    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 加载布局文件
        setContentView(R.layout.activity_main)

        // ProgressBar 进度条
        val mainProgressBar = findViewById<Button>(R.id.main_progress_bar)
        mainProgressBar.setOnClickListener {
            val intent = Intent(this, ProgressBarActivity::class.java)
            startActivity(intent)
        }

        // Toolbar 操作(标题)栏
        val mainToolbar = findViewById<Button>(R.id.main_toolbar)
        mainToolbar.setOnClickListener {
            val intent = Intent(this, ToolbarActivity::class.java)
            startActivity(intent)
        }

        // ListView 简单
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

        // Notification 通知栏
        val mainNotificationSimple = findViewById<Button>(R.id.main_notification_simple)
        mainNotificationSimple.setOnClickListener {
            val intent = Intent(this, NotificationSimpleActivity::class.java)
            startActivity(intent)
        }

    }

    // AlertDialog 对话框 的 点击事件
    fun MainAlertDialog(view: View) {
        val intent = Intent(this, AlertDialogActivity::class.java)
        startActivity(intent)
    }

    // PopupWindow 悬浮框 的 点击事件
    fun MainPopupWindow(view: View) {
        val intent = Intent(this, PopupWindowActivity::class.java)
        startActivity(intent)
    }

    // animation 动画
    fun MainAnimation(view: View) {
        val intent = Intent(this, AnimationActivity::class.java)
        startActivity(intent)
    }

    // ViewPager 多页面切换 1
    fun MainViewPager(view: View) {
        val intent = Intent(this, ViewPagerActivity::class.java)
        startActivity(intent)
    }

}