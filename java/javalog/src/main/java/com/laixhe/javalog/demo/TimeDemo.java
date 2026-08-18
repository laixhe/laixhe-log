package com.laixhe.javalog.demo;

import java.time.*;
import java.time.format.DateTimeFormatter;
import java.time.temporal.ChronoUnit;
import java.util.concurrent.Executors;
import java.util.concurrent.ScheduledExecutorService;
import java.util.concurrent.TimeUnit;

/**
 * 时间处理示例：获取当前时间、时间戳、格式化、解析、时区、耗时、定时器。
 * 对应 Go golog/time_test.go。
 *
 * 前置知识：
 * - LocalDateTime 本地时间（无时区）；Instant 时间戳（UTC 时刻）
 * - ZonedDateTime 带时区时间；DateTimeFormatter 线程安全，可复用
 * - 时区常量：ZoneOffset.UTC / ZoneOffset.ofHours(8)（东八区，对应 CST）
 *   ZoneId.of("Asia/Shanghai")（对应 PRC / Asia/Shanghai）
 */
public final class TimeDemo {

    private TimeDemo() {
    }

    // ============ 时间基本使用 ============
    public static void timeBasics() {
        // 获取当前时间对象
        Instant now = Instant.now();
        // 获取当前时间戳（秒）（对应 time.Now().Unix()）
        long epochSecond = now.getEpochSecond();
        // 当前时间戳（毫秒）
        long epochMilli = now.toEpochMilli();

        // 时间格式化（对应 time.Format("2006-01-02 15:04:05")）
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
        String formatted = LocalDateTime.now().format(fmt);

        // 使用 GMT 日期格式表示（对应 http.TimeFormat）
        String gmtFormat = ZonedDateTime.now(ZoneOffset.UTC).format(DateTimeFormatter.RFC_1123_DATE_TIME);

        // 将时间戳转为时间对象（对应 time.Unix(timeUnix, 0)）
        Instant fromEpoch = Instant.ofEpochSecond(epochSecond);

        System.out.println("获取当前时间对象: " + now);
        System.out.println("获取当前时间戳(秒): " + epochSecond);
        System.out.println("获取当前时间戳(毫秒): " + epochMilli);
        System.out.println("当前时间格式化: " + formatted);
        System.out.println("当前时间GMT格式化: " + gmtFormat);
        System.out.println("将时间戳转为时间对象: " + fromEpoch);
    }

    // ============ 时间字符串转换时间 ============
    public static void timeParse() {
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

        // 对应 time.Parse()：默认使用 UTC 时区解析
        Instant parsedUtc = LocalDateTime.parse("2018-01-06 16:12:00", fmt).toInstant(ZoneOffset.UTC);
        System.out.println("UTC 解析时间戳: " + parsedUtc.getEpochSecond()); // 1515255120

        // 对应 time.ParseInLocation(…, time.Local)：采用本地时区（东八区）
        Instant parsedLocal = LocalDateTime.parse("2018-01-06 16:12:00", fmt)
                .atZone(ZoneId.of("Asia/Shanghai"))
                .toInstant();
        System.out.println("本地(东八区)解析时间戳: " + parsedLocal.getEpochSecond()); // 1515226320

        // 时区相差 8 小时，时间戳不同
        System.out.println("相差秒数: " + (parsedUtc.getEpochSecond() - parsedLocal.getEpochSecond())); // 28800
    }

    // ============ 时间比较 ============
    public static void timeBeforeAfter() {
        Instant t1 = Instant.now().minus(1, ChronoUnit.SECONDS);
        Instant t2 = Instant.now().plus(1, ChronoUnit.HOURS);

        System.out.println("t2 是否在 t1 之前: " + t2.isBefore(t1)); // false
        System.out.println("t2 是否在 t1 之后: " + t2.isAfter(t1));  // true

        // 当前时间在范围内  t1 < * < t2
        Instant now = Instant.now();
        System.out.println(t1.isBefore(now) && t2.isAfter(now)); // true
    }

    // ============ 计算耗时或剩余时间 ============
    public static void timeSinceUntil() {
        // 计算耗时（对应 time.Since）
        Instant start = Instant.now();
        try {
            Thread.sleep(500); // 模拟耗时
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        }
        Duration elapsed = Duration.between(start, Instant.now());
        System.out.println("模拟耗时 " + elapsed.toMillis() + "ms"); // 约 500ms

        // 计算剩余时间（对应 time.Until）
        Instant deadline = Instant.now().plus(3, ChronoUnit.SECONDS);
        Duration remaining = Duration.between(Instant.now(), deadline);
        System.out.println("剩余时间 " + remaining.toMillis() + "ms"); // 约 3000ms
    }

    // ============ 自定义时区 ============
    public static void timeZone() {
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");

        // 单次设置：东八区（对应 time.FixedZone("CST", 8*3600)）
        ZoneOffset cstZone = ZoneOffset.ofHours(8);
        System.out.println("自定义时区1: " + ZonedDateTime.now(cstZone).format(fmt));

        // 命名时区（对应 time.LoadLocation("Asia/Shanghai")，不区分大小写）
        ZoneId cstSh = ZoneId.of("Asia/Shanghai");
        System.out.println("自定义时区2: " + ZonedDateTime.now(cstSh).format(fmt));

        // 将同一时刻按不同时区展示
        Instant now = Instant.parse("2025-06-21T09:18:39Z");
        System.out.println("UTC 时刻:   " + now.atZone(ZoneOffset.UTC).format(fmt));
        System.out.println("上海 时刻:  " + now.atZone(cstSh).format(fmt));

        // Asia/Shanghai       亚洲/上海
        // Asia/Chongqing      亚洲/重庆
        // UTC 世界协调时间（世界标准时间）
        // GMT 格林威治标准时间（UTC = GMT）
        // CST 代表多个时区：美国中部(UT-6) / 澳大利亚(UT+9:30) / 中国(UT+8) / 古巴(UT-4)
    }

    // ============ 定时器（周期性、单次）============
    // 对应 Go time.Tick（周期性）与 time.After（单次）
    public static void timeTicker() {
        ScheduledExecutorService scheduler = Executors.newScheduledThreadPool(2);
        try {
            // 周期性任务：每 1 秒执行一次（对应 time.Tick）
            scheduler.scheduleAtFixedRate(() ->
                    System.out.println("周期性: " + Instant.now().getEpochSecond()), 0, 1, TimeUnit.SECONDS);

            // 单次任务：3 秒后执行一次（对应 time.After）
            scheduler.schedule(() -> {
                System.out.println("单次: " + Instant.now().getEpochSecond());
                scheduler.shutdown(); // 结束后关闭，避免进程不退出
            }, 3, TimeUnit.SECONDS);

            // 等待单次任务完成（最多 5 秒）
            scheduler.awaitTermination(5, TimeUnit.SECONDS);
        } catch (InterruptedException e) {
            Thread.currentThread().interrupt();
        } finally {
            scheduler.shutdownNow();
        }
        System.out.println("定时器结束...");
    }
}
