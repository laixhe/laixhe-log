<?php

use Monolog\Logger;
use Monolog\Handler\StreamHandler;
use Yaf\Registry;

/**
 * 日志封装：基于 Monolog（纯 PHP，兼容 PHP 8.5）
 *
 * 日志路径在 conf/conf.ini 的 [log] 段配置：
 *   log.basepath = "/lai/logs"
 *   log.logger   = "api"
 *
 * 用法（与 SeasLog 类似）：
 *   Log::info('message');
 *   Log::error('message');
 */
class Log {

    /** @var Logger|null 单例 Logger 实例 */
    private static $logger = null;

    /**
     * 获取单例 Logger（懒加载，首次调用时初始化）
     *
     * 日志目录会自动创建；路径与名称优先从 Yaf 配置读取。
     *
     * @return Logger
     */
    private static function getLogger() : Logger {
        if (self::$logger === null) {
            // 默认值，可被 conf.ini 的 [log] 段覆盖
            $basepath = '/lai/logs';
            $loggerName = 'api';

            // 优先读取 Yaf 配置（Registry 未初始化时回退默认值）
            if (class_exists(Registry::class) && Registry::has('config')) {
                $config = Registry::get('config');
                if (isset($config->log->basepath)) {
                    $basepath = $config->log->basepath;
                }
                if (isset($config->log->logger)) {
                    $loggerName = $config->log->logger;
                }
            }

            // StreamHandler 不会自动创建目录，这里手动创建
            if (!is_dir($basepath)) {
                mkdir($basepath, 0755, true);
            }

            self::$logger = new Logger($loggerName);
            self::$logger->pushHandler(new StreamHandler($basepath . '/' . $loggerName . '.log', Logger::DEBUG));
        }
        return self::$logger;
    }

    /**
     * 记录错误日志
     */
    public static function error(string $message, array $context = []) : void {
        self::getLogger()->error($message, $context);
    }

    /**
     * 记录普通信息日志
     */
    public static function info(string $message, array $context = []) : void {
        self::getLogger()->info($message, $context);
    }

    /**
     * 记录调试日志
     */
    public static function debug(string $message, array $context = []) : void {
        self::getLogger()->debug($message, $context);
    }

    /**
     * 记录警告日志
     */
    public static function warning(string $message, array $context = []) : void {
        self::getLogger()->warning($message, $context);
    }
}
