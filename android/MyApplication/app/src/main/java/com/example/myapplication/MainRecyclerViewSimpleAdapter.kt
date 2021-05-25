package com.example.myapplication

import android.content.Context
import android.view.LayoutInflater
import android.view.ViewGroup
import androidx.recyclerview.widget.RecyclerView
import com.example.myapplication.databinding.ActivityMainRecyclerViewSimpleItemBinding

// RecyclerView 简单
class MainRecyclerViewSimpleAdapter(private var data: MutableList<MainRecyclerViewSimpleData>, private var context: Context):
    RecyclerView.Adapter<MainRecyclerViewSimpleAdapter.RecyclerViewHolder>() {

    override fun onCreateViewHolder(parent: ViewGroup, viewType: Int): RecyclerViewHolder {
        val itemBinding = ActivityMainRecyclerViewSimpleItemBinding.inflate(LayoutInflater.from(context), parent, false)
        return RecyclerViewHolder(itemBinding)
    }

    override fun onBindViewHolder(holder: RecyclerViewHolder, position: Int) {
        holder.itemBinding.recyclerViewSimpleItemTv.text = data[position].name
    }

    override fun getItemCount(): Int {
        return data.size
    }

    // 嵌套类
    inner class RecyclerViewHolder(val itemBinding: ActivityMainRecyclerViewSimpleItemBinding) : RecyclerView.ViewHolder(itemBinding.root) {
        init {
            itemBinding.recyclerViewSimpleItemTv.setOnClickListener {
                mOnRecyclerItemClickListener?.onRecyclerItemClick(bindingAdapterPosition)
            }
        }
    }

    // 用于回调
    private var mOnRecyclerItemClickListener: OnRecyclerItemClickListener? = null
    fun setRecyclerItemClickListener(listener: OnRecyclerItemClickListener){
        mOnRecyclerItemClickListener = listener
    }
    interface OnRecyclerItemClickListener {
        fun onRecyclerItemClick(position: Int)
    }
}