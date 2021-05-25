package com.example.myapplication

import android.content.Context
import android.view.LayoutInflater
import android.view.View
import android.view.ViewGroup
import android.widget.BaseAdapter

import com.example.myapplication.databinding.ActivityMainListViewSimpleItemBinding

// ListView 简单
class MainListViewSimpleAdapter(private var data: MutableList<MainListViewSimpleData>, private var context: Context) :
    BaseAdapter() {

    override fun getCount(): Int {
        return data.size
    }

    override fun getItem(position: Int): Any {
        return data[position]
    }

    override fun getItemId(position: Int): Long {
        return position.toLong()
    }

    override fun getView(position: Int, convertView: View?, parent: ViewGroup?): View {
        val itemBinding: ActivityMainListViewSimpleItemBinding?

        if (convertView == null) {
            itemBinding = ActivityMainListViewSimpleItemBinding.inflate(LayoutInflater.from(context), parent, false)
            itemBinding.root.tag = itemBinding
        } else {
            itemBinding = convertView.tag as ActivityMainListViewSimpleItemBinding
        }

        // 设置 TextView
        itemBinding.listViewSimpleItemTv.text = data[position].name

        if (convertView == null) {
            return itemBinding.root
        }
        return convertView
    }

}