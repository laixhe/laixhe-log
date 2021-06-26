#### Android 官网应用清单文件 AndroidManifest.xml
> https://developer.android.google.cn/guide/topics/manifest/manifest-intro

#### 组件声明(四大组件)
> 界面操作：android.app.Activity
>
> 后台服务：android.app.Service
>
> 广播通知：android.app.BroadcastReceive
>
> 数据交换：android.app.ContentProvider
>
> 将这四种交流方式所依赖的载体统一称为应用组件
>
> 应用组件是依赖于当前应用 Application 的，所以组件的生命周期只能小于等于当前应用生命周期

#### 应用程序 Application
> Android官网清单文件-application标签
>
> https://developer.android.google.cn/guide/topics/manifest/application-element
>
> Application 对应于清单文件中的 `<application></application>` 标签
>
> icon 属性加载资源文件下的图标，作为应用程序在 Android 系统的显示图标
>
> label 属性则是加载资源文件下的字符串，作为应用程序在 Android 系统中显示的应用名
>
> name 属性作为可填项，可以使用继承自 android.app.Application 的自定义类，如果不填该属性值，则默认为 android.app.Application

上面说到组件声明(四大组件)，每种组件在 `AndroidManifest.xml` 中对应的标签名，组件标签内必须要有 `name` 属性，以指向代码中自定义的组件类

| **组件名**      | **Activity**  | **Service** | **BroadcastReceiver** | **ContentProvider** |
| -------------- | ------------  | ----------- | --------------------- | ------------------- |
| 清单文件标签名    | activity     | service      | receiver              | provider            |

provider 标签中还必须要有 authorities 属性值，这是由于 ContentProvider 是对其他应用提供数据，这就好像该应用将数据保存到一个保险箱中提供给其他应用，而其他应用必须有该保险箱的密码才能打开保险箱以访问该应用的数据，而 authorities 正是起到这个密码的类似效果。

启动应用程序后，必须要加载一个 `Activity` 界面，而加载哪一个呢？这也就用到 `<intent-filter><\intent-filter>` 标签了，只有在该标签内嵌套了 `<action android:name="android.intent.action.MAIN" />` 和 `<category android:name="android.intent.category.LAUNCHER" />` 两个固定标签内容的 `<activity><\activity>`，才允许作为第一个界面加载。所以很自然的想到，一个 `<application><\application>` 标签内只允许有一个 `<activity><\activity>` 可以嵌套上面的意图过滤器内容。***如下面：***

```
<activity
    android:name=".MainActivity"
    android:exported="true">
    <intent-filter>
        <action android:name="android.intent.action.MAIN" />
        <category android:name="android.intent.category.LAUNCHER" />
    </intent-filter>
</activity>
```

每个应用程序对应 AndroidSDK 中的 android.app.Application 类，Application 的生命周期即从应用程序启动开始，当 Android 系统因为内存过低或电源优化时，被动杀死应用程序结束，也可能是应用程序主动退出结束。

#### 权限声明
> 应用程序可能需要联网操作，或者访问 Android 系统上的系统级应用中的内容，比如通讯录信息，这些可能涉及到用户隐私的操作和数据，统一归纳为应用权限
>
> 应用程序所使用到的权限都要在清单文件( `AndroidManifest.xml` )中以 `<uses-permisson />` 标签形式声明
>
> name 属性值中填入相关权限名
>
> 从 Android6.0 开始，应用程序用到的危险权限不仅要在清单文件中声明，而且在涉及该危险权限的代码调用之前也要动态检查申请。
>
> 可查询 Android 官网权限列表中 Protection level: dangerous 类型的权限即为危险权限：
>
> https://developer.android.google.cn/reference/android/Manifest.permission

在动态申请权限时，可使用 `checkSelfPermission(String permission)` 检查应用是否获得相关权限。如果用户没有授权，就需要使用 `requestPermission(@NonNull String[] permissions, int requestCode)` 申请相关权限。并在` Activity` 中重载 `onRequestPermissionsResult(int requestCode, @NonNull String[] permissions, @NonNull int[] grantResults)` 以获得用户对权限申请的处理结果。
