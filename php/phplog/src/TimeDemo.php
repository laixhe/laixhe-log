<?php

namespace Laixhe\Phplog;

use DateTime;
use DateTimeImmutable;
use DateTimeZone;

/**
 * 时间处理示例：获取当前时间、时间戳、格式化、解析、时区、耗时、定时器。
 * 对应 Go golog/time_test.go。
 *
 * 前置知识：
 * - date('Y-m-d H:i:s') 使用默认时区格式化时间戳
 * - DateTime / DateTimeImmutable 是面向对象的时间封装（Immutable 不可变，推荐）
 * - time() 返回秒级时间戳；microtime(true) 返回带毫秒的时间戳
 * - 时区：date_default_timezone_set 全局设置，或 new DateTimeZone('Asia/Shanghai')
 */
final class TimeDemo
{
    // ============ 时间基本使用 ============
    public static function timeBasics(): void
    {
        // 获取当前时间戳（秒）（对应 time.Now().Unix()）
        $timeUnix = time();
        // 获取当前时间戳（毫秒）
        $timeMilli = (int) (microtime(true) * 1000);
        // 当前时间对象
        $now = new DateTimeImmutable();

        // 时间格式化（对应 time.Format("2006-01-02 15:04:05")）
        $timeFormat = $now->format('Y-m-d H:i:s');
        // 使用 GMT 日期格式表示（对应 http.TimeFormat）
        $timeGmtFormat = $now->setTimezone(new DateTimeZone('UTC'))->format('D, d M Y H:i:s T');

        // 将时间戳转为时间对象（对应 time.Unix(timeUnix, 0)）
        $fromEpoch = (new DateTimeImmutable())->setTimestamp($timeUnix);

        echo "获取当前时间对象: {$now->format('Y-m-d H:i:s.u')}", PHP_EOL;
        echo "获取当前时间戳(秒): {$timeUnix}", PHP_EOL;
        echo "获取当前时间戳(毫秒): {$timeMilli}", PHP_EOL;
        echo "当前时间格式化: {$timeFormat}", PHP_EOL;
        echo "当前时间GMT格式化: {$timeGmtFormat}", PHP_EOL;
        echo "将时间戳转为时间对象: {$fromEpoch->format('Y-m-d H:i:s')}", PHP_EOL;
    }

    // ============ 时间字符串转换时间 ============
    public static function timeParse(): void
    {
        // 对应 time.Parse()：解析时使用 UTC 时区（默认）
        $parsedUtc = new DateTimeImmutable('2018-01-06 16:12:00', new DateTimeZone('UTC'));
        echo 'UTC 解析时间戳: ', $parsedUtc->getTimestamp(), PHP_EOL; // 1515255120

        // 对应 time.ParseInLocation(…, time.Local)：采用本地时区（东八区）
        $parsedLocal = new DateTimeImmutable('2018-01-06 16:12:00', new DateTimeZone('Asia/Shanghai'));
        echo '本地(东八区)解析时间戳: ', $parsedLocal->getTimestamp(), PHP_EOL; // 1515226320

        // 时区相差 8 小时，时间戳不同
        echo '相差秒数: ', $parsedUtc->getTimestamp() - $parsedLocal->getTimestamp(), PHP_EOL; // 28800

        // 自定义格式解析（对应 time.Parse 指定 layout）
        $custom = DateTimeImmutable::createFromFormat('Y-m-d H:i:s', '2018-01-06 16:12:00');
        echo 'createFromFormat: ', $custom->format('Y-m-d H:i:s'), PHP_EOL;
    }

    // ============ 时间比较 ============
    public static function timeBeforeAfter(): void
    {
        $now = new DateTimeImmutable();
        $t1 = $now->modify('-1 second');
        $t2 = $now->modify('+1 hour');

        // DateTime 对象之间直接比较（对应 t2.Before(t1) / After）
        var_dump($t2 < $t1); // false  t2 是否在 t1 之前  =>  t2 < t1
        var_dump($t2 > $t1); // true   t2 是否在 t1 之后  =>  t2 > t1

        // 当前时间在范围内  t1 < * < t2
        var_dump($t1 < $now && $now < $t2); // true
    }

    // ============ 计算耗时或剩余时间 ============
    public static function timeSinceUntil(): void
    {
        // 计算耗时（对应 time.Since）
        $start = microtime(true);      // 开始时间（浮点秒）
        usleep(500_000);               // 模拟耗时 0.5 秒
        $elapsed = microtime(true) - $start;
        printf('模拟耗时 %.0fms' . PHP_EOL, $elapsed * 1000); // 约 500ms

        // 计算剩余时间（对应 time.Until）
        $deadline = new DateTimeImmutable('+3 seconds');
        $remaining = $deadline->getTimestamp() - time();
        echo "剩余时间 {$remaining}s", PHP_EOL; // 约 3
    }

    // ============ 自定义时区 ============
    public static function timeZone(): void
    {
        $fmt = 'Y-m-d H:i:s';

        // 单次设置：东八区（对应 time.FixedZone("CST", 8*3600)）
        $cstZone = new DateTimeZone('+08:00');
        echo '自定义时区1: ', (new DateTimeImmutable('now', $cstZone))->format($fmt), PHP_EOL;

        // 命名时区（对应 time.LoadLocation("Asia/Shanghai")）
        $cstSh = new DateTimeZone('Asia/Shanghai');
        echo '自定义时区2: ', (new DateTimeImmutable('now', $cstSh))->format($fmt), PHP_EOL;

        // 将同一时刻按不同时区展示
        $now = new DateTimeImmutable('2025-06-21 09:18:39', new DateTimeZone('UTC'));
        echo 'UTC 时刻:   ', $now->format($fmt), PHP_EOL;
        echo '上海 时刻:  ', $now->setTimezone($cstSh)->format($fmt), PHP_EOL;

        // Asia/Shanghai       亚洲/上海
        // Asia/Chongqing      亚洲/重庆
        // UTC 世界协调时间（世界标准时间）
        // GMT 格林威治标准时间（UTC = GMT）
        // CST 代表多个时区：美国中部(UT-6) / 澳大利亚(UT+9:30) / 中国(UT+8) / 古巴(UT-4)
    }

    // ============ 定时器（周期性、单次）============
    // 对应 Go time.Tick（周期性）与 time.After（单次）
    // PHP CLI 没有内置定时器，用循环 + usleep 模拟（异步定时器需要 Swoole/ReactPHP 等）
    public static function timeTicker(): void
    {
        $start = time();
        $deadline = $start + 3; // 3 秒后停止

        while (true) {
            // 周期性任务：每 1 秒执行一次（对应 time.Tick）
            echo '周期性: ', time(), PHP_EOL;
            if (time() >= $deadline) {
                // 单次任务：3 秒后执行一次（对应 time.After）
                echo '单次: ', time(), PHP_EOL;
                break;
            }
            sleep(1);
        }
        echo '定时器结束...', PHP_EOL;
    }
}
