//
buildscript {

    val compose_version by extra("1.0.2")

    repositories {
        google()
        mavenCentral()
    }

    dependencies {
        classpath("com.android.tools.build:gradle:7.0.2")
        classpath("org.jetbrains.kotlin:kotlin-gradle-plugin:1.5.21")
    }

}

tasks.register("clean", Delete::class) {
    delete(rootProject.buildDir)
}

//
// java
//
//buildscript {
//    repositories {
//        google()
//        mavenCentral()
//    }
//    dependencies {
//        classpath "com.android.tools.build:gradle:7.0.0"
//        classpath "org.jetbrains.kotlin:kotlin-gradle-plugin:1.5.20"
//    }
//}
//
//task clean(type: Delete) {
//    delete rootProject.buildDir
//}
//