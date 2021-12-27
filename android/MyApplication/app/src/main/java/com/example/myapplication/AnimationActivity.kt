package com.example.myapplication

import android.animation.ObjectAnimator
import android.animation.ValueAnimator
import android.graphics.drawable.AnimationDrawable
import android.os.Bundle
import android.util.Log
import android.view.animation.AnimationUtils
import androidx.appcompat.app.AppCompatActivity
import com.example.myapplication.databinding.ActivityAnimationBinding

// animation 动画
class AnimationActivity: AppCompatActivity() {
    // 视图绑定
    private lateinit var binding: ActivityAnimationBinding

    private var flag: Boolean = true

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)

        // 通过视图绑定来加载布局文件
        binding = ActivityAnimationBinding.inflate(layoutInflater)
        setContentView(binding.root)

        // 帧动画 播放
        val anim: AnimationDrawable = binding.animationFrame.background as AnimationDrawable
        binding.animationFrame.setOnClickListener {
            if (flag) {
                flag = !flag
                // 开始动画
                anim.start()
            } else {
                flag = !flag
                // 停止动画
                anim.stop()
            }
        }

        // 补间动画
        binding.animationTween.setOnClickListener {
            // 通过加载 animation_tween_xxx.xml 动画设置文件来创建一个 Animation 对象
            //val animation = AnimationUtils.loadAnimation(this, R.anim.animation_tween_alpha)
            val animation = AnimationUtils.loadAnimation(this, R.anim.animation_tween_scale)
            // 启动补间动画
            binding.animationTween.startAnimation(animation)
        }

        // 属性动画 1
        val valueAnimator = ValueAnimator.ofFloat(0f, 1f)
        // 持续时间毫秒
        valueAnimator.duration = 2000
        // 监听动画播放过程
        valueAnimator.addUpdateListener {
            val v: Float = it.animatedValue as Float
            Log.e("T------", "valueAnimator addUpdateListener v=$v")
        }
        valueAnimator.start()

        // 属性动画 2
        val objectAnimator = ObjectAnimator.ofFloat(binding.animationTween, "alpha", 0f, 1f)
        objectAnimator.duration = 6000
        objectAnimator.start()
    }
}