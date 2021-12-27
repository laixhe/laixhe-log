package com.example.myapplication

import android.view.View
import android.view.ViewGroup
import androidx.viewpager.widget.PagerAdapter

// ViewPager 多页面切换 1
class ViewPagerAdapter(private val mListView: ArrayList<View>): PagerAdapter() {

    override fun instantiateItem(container: ViewGroup, position: Int): Any {
        container.addView(mListView[position], 0)
        return mListView[position]
    }

    override fun getCount(): Int {
        return mListView.size
    }

    override fun isViewFromObject(view: View, `object`: Any): Boolean {
        return view == `object`
    }

    override fun destroyItem(container: ViewGroup, position: Int, `object`: Any) {
        container.removeView(mListView[position])
    }

}