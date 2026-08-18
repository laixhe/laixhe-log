package com.laixhe.javalog;

import com.laixhe.javalog.demo.TimeDemo;
import org.junit.jupiter.api.Test;

import java.time.*;
import java.time.format.DateTimeFormatter;
import java.time.temporal.ChronoUnit;

import static org.junit.jupiter.api.Assertions.*;

/**
 * 时间处理测试（对应 Go time_test.go 的核心断言）。
 */
class TimeTest {

    // 练习 1：时间格式化与解析（对应 Go TestTime / TestTimeParse）
    @Test
    void exercise1_format_parse() {
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
        // 格式化
        String formatted = LocalDateTime.of(2025, 6, 21, 17, 18, 39).format(fmt);
        assertEquals("2025-06-21 17:18:39", formatted);

        // 解析（默认本地时区语义演示）
        LocalDateTime parsed = LocalDateTime.parse("2018-01-06 16:12:00", fmt);
        assertEquals(2018, parsed.getYear());
        assertEquals(1, parsed.getMonthValue());
    }

    // 练习 2：时区差异（对应 Go TestTimeParse 中 Parse 与 ParseInLocation）
    @Test
    void exercise2_timezone_offset() {
        DateTimeFormatter fmt = DateTimeFormatter.ofPattern("yyyy-MM-dd HH:mm:ss");
        Instant utc = LocalDateTime.parse("2018-01-06 16:12:00", fmt).toInstant(ZoneOffset.UTC);
        Instant local = LocalDateTime.parse("2018-01-06 16:12:00", fmt)
                .atZone(ZoneId.of("Asia/Shanghai"))
                .toInstant();
        // 东八区比 UTC 早 8 小时，UTC 时间戳更大
        assertEquals(8 * 3600, utc.getEpochSecond() - local.getEpochSecond());
    }

    // 练习 3：时间比较（对应 Go TestTimeBeforeAfter）
    @Test
    void exercise3_before_after() {
        Instant t1 = Instant.now().minus(1, ChronoUnit.SECONDS);
        Instant t2 = Instant.now().plus(1, ChronoUnit.HOURS);
        assertFalse(t2.isBefore(t1));
        assertTrue(t2.isAfter(t1));

        Instant now = Instant.now();
        assertTrue(t1.isBefore(now) && t2.isAfter(now));
    }

    // 练习 4：耗时计算（对应 Go TestTimeSinceUntil）
    @Test
    void exercise4_duration() throws InterruptedException {
        Instant start = Instant.now();
        Thread.sleep(50);
        Duration elapsed = Duration.between(start, Instant.now());
        assertTrue(elapsed.toMillis() >= 50);

        Instant deadline = Instant.now().plus(3, ChronoUnit.SECONDS);
        Duration remaining = Duration.between(Instant.now(), deadline);
        assertTrue(remaining.getSeconds() <= 3);
    }

    // 运行完整 Demo（timeTicker 会阻塞 3 秒，单独运行）
    @Test
    void runTimeDemo() {
        TimeDemo.timeBasics();
        TimeDemo.timeParse();
        TimeDemo.timeBeforeAfter();
        TimeDemo.timeSinceUntil();
        TimeDemo.timeZone();
    }

    @Test
    void runTimeTicker() {
        TimeDemo.timeTicker(); // 周期性 3 秒后结束
    }
}
