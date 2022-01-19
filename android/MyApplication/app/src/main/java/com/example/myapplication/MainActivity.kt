package com.example.myapplication

import android.content.Intent
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import android.widget.Button
import android.widget.Toast

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

        // Button 按钮
        val mainButtonSimple = findViewById<Button>(R.id.main_button_simple)
        mainButtonSimple.setOnClickListener {
            val intent = Intent(this, ButtonSimpleActivity::class.java)
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

        // C++
        Toast.makeText(this, "C++: "+stringFromJNI(), Toast.LENGTH_LONG).show()
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

    // Menu 简单使用
    fun MainMenu(view: View) {
        val intent = Intent(this, MenuSimpleActivity::class.java)
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

    // Fragment 简单应用
    fun MainFragmentSimple(view: View) {
        val intent = Intent(this, FragmentSimpleActivity::class.java)
        startActivity(intent)
    }

    // 读写内部文件存储-简单使用
    fun MainFileSimple(view: View) {
        val intent = Intent(this, FileSimpleActivity::class.java)
        startActivity(intent)
    }

    // SharedPreferences 存储
    fun MainSharedPreferences(view: View) {
        val intent = Intent(this, SharedPreferencesActivity::class.java)
        startActivity(intent)
    }

    // SQLite 数据库-简单使用
    fun MainSQLiteSimple(view: View) {
        val intent = Intent(this, SQLiteSimpleActivity::class.java)
        startActivity(intent)
    }

    // DrawerLayout 侧滑菜单-简单使用
    fun MainDrawerLayoutSimple(view: View) {
        val intent = Intent(this, DrawerLayoutSimpleActivity::class.java)
        startActivity(intent)
    }

    // NavigationView 侧滑菜单-简单使用
    fun MainNavigationViewSimple(view: View) {
        val intent = Intent(this, NavigationViewSimpleActivity::class.java)
        startActivity(intent)
    }


    external fun stringFromJNI(): String

    companion object {
        // Used to load the 'myapplication' library on application startup.
        init {
            System.loadLibrary("myapplication")
        }
    }
}