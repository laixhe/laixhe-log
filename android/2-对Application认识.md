Android 系统启动应用的时候，会首先加载 `AndroidManifest.xml` 清单文件中的一系列信息，在清单文件中如果不指定 **<application></application>** 标签中的 **name** 属性值，会默认使用 `android.app.Application` 作为应用程序加载类，其生命周期与应用程序生命周期是一致的。

#### 生命周期

##### 应用实例化

首先要知道，一个应用程序，有且仅有一个 [android.app.Application](https://developer.android.google.cn/reference/android/app/Application) 类与之对应，如果想在清单文件中使用自定义的 `Application`，也必须是继承自 `android.app.Application` 的子类。`Application` 的实例在Android系统启动应用时优先于应用中其他任何类而创建。

##### 应用创建

当 `Application` 类与上下文环境绑定后，说明该应用程序已经加载运行环境，Android系统会继续调用 `onCreate()` 方法，表明该应用程序已创建。所以自定义的 `Application` 可以重载该方法，以完成在应用程序创建初期就要执行的操作。一般应用中使用的第三方框架都会在此处初始化，以保证应用创建之时就初始化框架结构。在重载该方法时，一定要优先调用其父类方法 `super.onCreate()`。

##### 应用硬件改变

在应用程序运行过程中，当Android系统所搭载的硬件设备发生变化时，Android 系统会调用该类的 `onConfigurationChanged(Configuration newConfig)` 方法。如果重载该方法，可以响应 [android.content.res.Configuration](https://developer.android.google.cn/reference/android/content/res/Configuration) 类中所涉及到的硬件设备变化，例如常见的横竖屏切换、应用夜间主题与正常主题切换等，都可以在这里处理。另外，在重载该方法时，一定要优先调用其父类方法 `super.onConfigurationChanged(newConfig)`。

##### 应用低内存警告

Android系统运行内存空间使用殆尽，可能导致应用程序无法正常运行。而此时当前应用程序如果还在前台运行，Android系统会调用该类的 `onLowMemory()` 方法，之后会杀死应用程序中的后台服务。如果重载该方法，可以释放一些不需要的资源，以防止内存空间进一步缩减，将可能保留前台界面。同样，在重载该方法时，一定要优先调用其父类方法 `super.onLowMemory()`。

##### 应用内存杀死

而当Android系统运行内存空间使用殆尽时，应用程序也可能恰好在后台运行，此时Android系统会调用该类的 `onTrimMemory(int level)` 方法，以表明应用程序即将被Android系统杀死。其中的level参数可以表明当前应用程序所处等级，level等级低的将优先被Android系统完全杀死。如果重载该方法，则不是释放资源就能解决的了，可以执行一些数据持久化保存操作，防止应用程序被杀死后再次启动后出现数据不一致的情况。同样，在重载该方法时，一定要优先调用其父类方法 `super.onTrimMemory(level)`。值得注意的是，应用程序被用户主动杀死时，在该类中是没有方法响应的。

##### 其它

保存应用程序全局变量使用。不推荐该方式，因为当应用程序中含有多个进程时，每个进程在创建时都会开辟新的内存空间，同时在新内存中创建新的 `Application` 实例与之对应，不同进程中的全局变量也就不一致了。
