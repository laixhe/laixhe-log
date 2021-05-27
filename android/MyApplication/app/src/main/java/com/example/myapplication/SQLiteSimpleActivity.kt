package com.example.myapplication

import android.content.ContentValues
import android.content.Context
import android.database.sqlite.SQLiteDatabase
import android.database.sqlite.SQLiteOpenHelper
import android.os.Bundle
import android.util.Log
import android.widget.Toast
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivitySqliteSimpleBinding

// SQLite 数据库-简单使用
class SQLiteSimpleActivity: AppCompatActivity() {

    // 视图绑定
    private lateinit var binding: ActivitySqliteSimpleBinding

    private val createBook = """
        CREATE TABLE Book (
        	id INTEGER PRIMARY KEY AUTOINCREMENT,
        	author TEXT(20) DEFAULT '' NOT NULL,
        	price REAL DEFAULT 0 NOT NULL,
        	pages INTEGER DEFAULT 0 NOT NULL,
        	name TEXT(50) DEFAULT '' NOT NULL
        )
    """.trimIndent()

    private val createCategory = """
        CREATE TABLE Category (
        	cid INTEGER PRIMARY KEY AUTOINCREMENT,
        	c_name TEXT(20) DEFAULT '' NOT NULL,
        	c_code INTEGER DEFAULT 0 NOT NULL
        );
    """.trimIndent()

    private lateinit var testSQLite: TestSQLite

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        // 通过视图绑定来加载布局文件
        binding = ActivitySqliteSimpleBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // 创建数据库
        testSQLite = TestSQLite(this, "book.db", null, 2)
        // 获取到数据库的操作
        val db = testSQLite.writableDatabase

        binding.sqliteSimpleCreate.setOnClickListener {
            // 新增一条
            val v = ContentValues()
            v.put("name", "第2本")
            v.put("author", "laixhe")
            v.put("price", 19)
            v.put("pages", 889)

            db.insert("Book", null, v)
            v.clear()
        }

        binding.sqliteSimpleUpdate.setOnClickListener {
            // 修改一条
            val v = ContentValues()
            v.put("pages", 999)

            db.update("Book", v, "id=?", arrayOf("1"))
        }

        binding.sqliteSimpleSelect.setOnClickListener {
            // 查询
            val cursor = db.query("Book", null, null, null, null, null, null, null)
            while (cursor.moveToNext()){
                val name = cursor.getString(cursor.getColumnIndex("name"))
                Log.e("T-----", name)
            }
        }
    }

    // 内部类
    inner class TestSQLite(private val context: Context, val name: String, private val factory: SQLiteDatabase.CursorFactory?, val version: Int):
        SQLiteOpenHelper(context, name, factory, version){

        // 当数据库已经存在就不在执行了
        override fun onCreate(db: SQLiteDatabase?) {
            if (db != null) {
                db.execSQL(this@SQLiteSimpleActivity.createBook)

                Toast.makeText(context, "onCreate: ${db.version}", Toast.LENGTH_LONG).show()
            }
        }

        // 只版本发生改变时
        override fun onUpgrade(db: SQLiteDatabase?, oldVersion: Int, newVersion: Int) {
            if (db != null) {
                if (db.version == 1) {
                    db.execSQL(this@SQLiteSimpleActivity.createCategory)
                }

                Toast.makeText(context, "onUpgrade: ${db.version}", Toast.LENGTH_LONG).show()
            }
        }

    }
}