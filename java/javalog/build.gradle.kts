plugins {
    java
    application
}

group = "com.laixhe"
version = "1.0.0"

// 使用 GraalVM 25 / JDK 25 工具链编译与运行
java {
    toolchain {
        languageVersion = JavaLanguageVersion.of(25)
    }
}

// 入口主类（对应 Rust 的 main.rs / Go 的 main 包）
application {
    mainClass.set("com.laixhe.javalog.Main")
}

repositories {
    mavenCentral()
}

dependencies {
    // JSON 序列化（对应 Go encoding/json / Rust serde_json）
    implementation("com.fasterxml.jackson.core:jackson-databind:2.18.2")
    // 时间类型 (Instant/LocalDateTime) 序列化支持（对应 Go omitzero 中 time.Time 场景）
    implementation("com.fasterxml.jackson.datatype:jackson-datatype-jsr310:2.18.2")

    // JUnit 5（对应 Go *_test.go / Rust #[cfg(test)]）
    testImplementation(platform("org.junit:junit-bom:5.11.4"))
    testImplementation("org.junit.jupiter:junit-jupiter")
    testRuntimeOnly("org.junit.platform:junit-platform-launcher")
}

tasks.test {
    useJUnitPlatform()
    testLogging {
        // 控制台直接展示测试输出（类似 go test -v）
        showStandardStreams = true
        events("passed", "skipped", "failed")
        showExceptions = true
        showCauses = true
    }
}

// Windows 控制台默认代码页为 GBK，强制 JVM 以 UTF-8 输出，避免中文乱码
tasks.withType<JavaExec> {
    jvmArgs(
        "-Dfile.encoding=UTF-8",
        "-Dsun.stdout.encoding=UTF-8",
        "-Dsun.stderr.encoding=UTF-8"
    )
}

tasks.withType<JavaCompile> {
    options.encoding = "UTF-8"
    // 保留参数名，供 Jackson 等框架反射使用
    options.compilerArgs.add("-parameters")
}
