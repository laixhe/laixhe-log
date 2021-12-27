package com.example.myapplication

import android.os.Bundle
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityFileSimpleBinding
import java.io.*

// 读写内部文件存储-简单使用
// 文件存放在 /data/data/xxx/files 目录下
class FileSimpleActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivityFileSimpleBinding

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityFileSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // 写入文件
        binding.fileSimpleEtSet.setOnClickListener {
            Toast.makeText(this, binding.fileSimpleEt.text.toString(), Toast.LENGTH_SHORT).show()

            try {
                val fileOutputStream = openFileOutput("file_simple.txt", MODE_PRIVATE)
                val bfs = BufferedWriter(OutputStreamWriter(fileOutputStream))
                bfs.use {
                    it.write(binding.fileSimpleEt.text.toString())
                }
            } catch (e: IOException){
                e.printStackTrace()
            }

        }

        // 读取文件
        binding.fileSimpleEtGet.setOnClickListener {
            val data = StringBuilder()

            try {
            val fileInputStream = openFileInput("file_simple.txt")
            val bis = BufferedReader(InputStreamReader(fileInputStream))
            bis.use { it ->
                it.forEachLine { ite ->
                    data.append(ite)
                }
            }
            } catch (e: Exception) {
                e.printStackTrace()
            }

            Toast.makeText(this, data.toString(), Toast.LENGTH_LONG).show()
        }

    }

    // use 是 kotlin 的一个内置函数，它保证在 lambda 表达式结束的时候会自动的流关闭

}