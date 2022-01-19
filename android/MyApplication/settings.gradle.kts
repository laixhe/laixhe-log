//
dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        jcenter() // 警告：此存储库即将关闭
    }
}
rootProject.name = "My Application"
include(":app")

//
// java
//
//dependencyResolutionManagement {
//    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
//    repositories {
//        google()
//        mavenCentral()
//        jcenter()
//    }
//}
//rootProject.name = "My Application"
//include ':app'
//