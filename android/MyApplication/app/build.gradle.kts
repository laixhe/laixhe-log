// 引用插件
plugins {
    id("com.android.application")
    id("kotlin-android")
}

// Android 属性
android {
    compileSdk = 30
    buildToolsVersion = "30.0.3"

    defaultConfig {
        applicationId = "com.example.myapplication"
        minSdk = 21
        targetSdk = 30
        // 可以在 Android 系统的设置-应用设置-该应用信息中展示
        versionCode = 1
        // 只能在应用内部显示调用展示
        versionName = "1.0"

        //
        vectorDrawables {
           useSupportLibrary = true
        }

        externalNativeBuild {
            cmake {
                cppFlags += "-std=c++17"
                // 设置支持的 so 库构架 "armeabi-v7a" , "arm64-v8a", "x86", "x86_64"
                abiFilters += mutableListOf("armeabi-v7a", "arm64-v8a", "x86_64")
                //abiFilters += "armeabi-v7a"
                //abiFilters += "arm64-v8a"
            }
        }

        //testInstrumentationRunner = "androidx.test.runner.AndroidJUnitRunner"
    }

    // 编译类型
    buildTypes {
        release {
            // 是否代码混淆
            isMinifyEnabled = false
            proguardFiles(
                getDefaultProguardFile("proguard-android-optimize.txt"),
                "proguard-rules.pro"
            )
        }
    }

    // 编译选项操作
    compileOptions {
        sourceCompatibility = JavaVersion.VERSION_1_8
        targetCompatibility = JavaVersion.VERSION_1_8
    }

    kotlinOptions {
        jvmTarget = "1.8"
		//useIR = true
    }

    externalNativeBuild {
        cmake {
            path = file("src/main/cpp/CMakeLists.txt")
            version = "3.10.2" // 3.18.1
        }
    }

    // 构建功能
    buildFeatures {
        // 开启视图绑定
        viewBinding = true
        // 开启声明式UI
        compose = true
    }

    composeOptions {
        kotlinCompilerExtensionVersion = rootProject.extra["compose_version"] as String
    }

//    packagingOptions{
//        resources {
//            excludes += "/META-INF/{AL2.0,LGPL2.1}"
//        }
//    }

}

// 依赖
dependencies {
    implementation("androidx.core:core-ktx:1.6.0")
    implementation("androidx.appcompat:appcompat:1.3.1")
    implementation("com.google.android.material:material:1.4.0")
    implementation("androidx.constraintlayout:constraintlayout:2.1.0")
    implementation("androidx.recyclerview:recyclerview:1.2.1")
    
	implementation("androidx.lifecycle:lifecycle-runtime-ktx:2.3.1")
    implementation("androidx.lifecycle:lifecycle-livedata-ktx:2.3.1")
    implementation("androidx.lifecycle:lifecycle-viewmodel-ktx:2.3.1")
    implementation("androidx.navigation:navigation-fragment-ktx:2.3.5")
    implementation("androidx.navigation:navigation-ui-ktx:2.3.5")

    implementation("androidx.compose.ui:ui:${rootProject.extra["compose_version"]}")
    implementation("androidx.compose.material:material:${rootProject.extra["compose_version"]}")
    implementation("androidx.compose.ui:ui-tooling-preview:${rootProject.extra["compose_version"]}")
    implementation("androidx.activity:activity-compose:1.3.1")

    implementation("com.squareup.okhttp3:okhttp:4.9.1")
    implementation("de.hdodenhof:circleimageview:3.1.0")

    //testImplementation("junit:junit:4.+")
    //androidTestImplementation("androidx.test.ext:junit:1.1.3")
    //androidTestImplementation("androidx.test.espresso:espresso-core:3.4.0")
    //androidTestImplementation("androidx.compose.ui:ui-test-junit4:${rootProject.extra["compose_version"]}")
	//debugImplementation("androidx.compose.ui:ui-tooling:${rootProject.extra["compose_version"]}")
}

//
// java
//
//plugins {
//    id 'com.android.application'
//    id 'kotlin-android'
//}
//
//android {
//    compileSdk 30
//    buildToolsVersion "30.0.3"
//
//    defaultConfig {
//        applicationId "com.example.myapplication"
//        minSdk 21
//        targetSdk 30
//        versionCode 1
//        versionName "1.0"
//
//        testInstrumentationRunner "androidx.test.runner.AndroidJUnitRunner"
//
//        externalNativeBuild {
//            cmake {
//                cppFlags '-std=c++17'
//                abiFilters 'armeabi-v7a', 'arm64-v8a'
//            }
//        }
//
//    }
//
//    buildTypes {
//        release {
//            minifyEnabled false
//            proguardFiles getDefaultProguardFile('proguard-android-optimize.txt'), 'proguard-rules.pro'
//        }
//    }
//
//    externalNativeBuild {
//        cmake {
//            path file('src/main/cpp/CMakeLists.txt')
//            version '3.10.2'
//        }
//    }
//
//    compileOptions {
//        sourceCompatibility JavaVersion.VERSION_1_8
//        targetCompatibility JavaVersion.VERSION_1_8
//    }
//
//    kotlinOptions {
//        jvmTarget = '1.8'
//    }
//
//    buildFeatures {
//        viewBinding true
//    }
//}
//
//dependencies {
//    implementation 'androidx.core:core-ktx:1.5.0'
//    implementation 'androidx.appcompat:appcompat:1.3.0'
//    implementation 'com.google.android.material:material:1.3.0'
//    implementation 'androidx.constraintlayout:constraintlayout:2.0.4'
//    testImplementation 'junit:junit:4.+'
//    androidTestImplementation 'androidx.test.ext:junit:1.1.2'
//    androidTestImplementation 'androidx.test.espresso:espresso-core:3.3.0'
//}
//
