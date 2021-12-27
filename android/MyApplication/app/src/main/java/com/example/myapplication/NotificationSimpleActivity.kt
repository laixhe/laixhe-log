package com.example.myapplication

import android.app.Notification
import android.app.NotificationChannel
import android.app.NotificationManager
import android.app.PendingIntent
import android.content.Intent
import android.graphics.BitmapFactory
import android.graphics.Color
import android.os.Build
import androidx.appcompat.app.AppCompatActivity
import android.os.Bundle
import android.view.View
import androidx.core.app.NotificationCompat

// Notification 通知栏
class NotificationSimpleActivity : AppCompatActivity() {

    private lateinit var manager: NotificationManager
    private lateinit var notification: Notification

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_notification_simple)

        // getSystemService 是 Android 很重要的一个API，它是 Activity 的一个方法
        // 根据传入的 NAME 来取得对应的 Object，然后转换成相应的服务对象
        manager = getSystemService(NOTIFICATION_SERVICE) as NotificationManager

        // 以下介绍系统相应的服务
        /**
         * 传入的 Name               |         返回的对象            |           说明
         * WINDOW_SERVICE                   WindowManager                 管理打开的窗口程序
         * LAYOUT_INFLATER_SERVICE          LayoutInflater                取得xml里定义的view
         * ACTIVITY_SERVICE                 ActivityManager               管理应用程序的系统状态
         * POWER_SERVICE                    PowerManger                   电源的服务
         * ALARM_SERVICE                    AlarmManager                  闹钟的服务
         * NOTIFICATION_SERVICE             NotificationManager           状态栏的服务
         * KEYGUARD_SERVICE                 KeyguardManager               键盘锁的服务
         * LOCATION_SERVICE                 LocationManager               位置的服务，如GPS
         * SEARCH_SERVICE                   SearchManager                 搜索的服务
         * VEBRATOR_SERVICE                 Vebrator                      手机震动的服务
         * CONNECTIVITY_SERVICE             Connectivity                  网络连接的服务
         * WIFI_SERVICE                     WifiManager                   Wi-Fi服务
         * TELEPHONY_SERVICE                TeleponyManager               电话服务
         */

        // 通知重要程度设置 NotificationManager 类中
        // IMPORTANCE_NONE    关闭通知
        // IMPORTANCE_MIN     开启通知，不会弹出，但没有提示音，状态栏中无显示
        // IMPORTANCE_LOW     开启通知，不会弹出，不发出提示音，状态栏中显示
        // IMPORTANCE_DEFAULT 开启通知，不会弹出，发出提示音，状态栏中显示 (默认)
        // IMPORTANCE_HIGH    开启通知，会弹出，发出提示音，状态栏中显示

        // 只在 Android 8.0 之上需要通知渠道 - 要适配
        if (Build.VERSION.SDK_INT >= Build.VERSION_CODES.O) {
            val channel = NotificationChannel("leo", "测试通知", NotificationManager.IMPORTANCE_HIGH)
            manager.createNotificationChannel(channel)
        }

        // 点击通知跳转页面
        val intent = Intent(this, ProgressBarActivity::class.java)
        val pendingIntent = PendingIntent.getActivity(this, 0, intent, 0)

        notification = NotificationCompat.Builder(this, "leo")
            .setContentTitle("官方通知") // 标题 必须的
            .setContentText("世界那么大，想去走走!") // 文本内容 必须的
            .setSmallIcon(R.drawable.abc_vector_test) // 小图标，只能使用 alpha 图层，不能包括 RGB 图层(不能有颜色) 必须的
            .setColor(Color.parseColor("#ff0000")) // 小图标颜色
            .setLargeIcon(BitmapFactory.decodeResource(resources, R.drawable.golang_code)) // 大图标
            .setContentIntent(pendingIntent) // 点击通知跳转页面
            .setAutoCancel(true) // 点击通知后自动清除通知
            .build()
    }

    // 发出通知
    fun sendNotification(view: View) {
        manager.notify(1, notification)
    }

    // 取消通知
    fun cancelNotification(view: View) {
        manager.cancel(1)
    }

    // 注意：上面的 ID 必须一样
}