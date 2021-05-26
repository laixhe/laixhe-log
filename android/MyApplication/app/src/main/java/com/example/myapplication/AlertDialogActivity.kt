package com.example.myapplication

import android.app.ProgressDialog
import android.os.Bundle
import android.util.Log
import android.view.View
import androidx.appcompat.app.AlertDialog
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityAlertDialogBinding

// AlertDialog 对话框
class AlertDialogActivity: AppCompatActivity() {

    // AlertDialog 不能指定显示位置，只能默认显示在屏幕最中间
    // 而 PopupWindow 悬浮框 是可以指定显示位置

    // 视图绑定
    private lateinit var binding: ActivityAlertDialogBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityAlertDialogBinding.inflate(layoutInflater)
        setContentView(binding.root)

        binding.alertDialogCode.setOnClickListener {
            // 使用代码创建对话框
            val alertDialog = AlertDialog.Builder(this)
            alertDialog.setIcon(R.mipmap.ic_launcher)

            //alertDialog.setTitle("对话标题")
            //alertDialog.setMessage("对话框内容...")

            //val dialogView = layoutInflater.inflate(R.layout.activity_alert_dialog_view, null)
            alertDialog.setView(R.layout.activity_alert_dialog_view)

            alertDialog.setPositiveButton("确定") { dialog, which ->
                Log.e("T--------", "setPositiveButton")
            }
            alertDialog.setNegativeButton("取消") { dialog, which ->
                Log.e("T--------", "setNegativeButton")
            }

            alertDialog.create()
            alertDialog.show()
        }
    }

    fun progressDialogCode(view: View) {
        val progressDialog = ProgressDialog(this)
        progressDialog.setTitle("标题")
        progressDialog.setMessage("内容...")
        progressDialog.show()
    }

}