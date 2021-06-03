<?php

//function appErrorHandler($errno, $errstr, $errfile, $errline)
//{
//    print_r($errno);
//    echo "<br>";
//    print_r($errstr);
//    echo "<br>";
//    print_r($errfile);
//    echo "<br>";
//    print_r($errline);
//}

// 设置页面内容、编码格式
header('Content-Type: text/html;charset=utf-8');
//header('Content-Type: application/json');

// 设置时区
date_default_timezone_set('PRC');

// 根目录
define('APPLICATION_PATH', dirname(__DIR__));

// 加载配置项
$app  = new Yaf\Application(APPLICATION_PATH . "/conf/conf.ini");

// 在出错的时候, 是否抛出异常
$app->getDispatcher()->catchException(true);
$app->getDispatcher()->throwException(true);
// 在入口文件中设置错误捕获函数
//$app->getDispatcher()->throwException(false)->setErrorHandler('appErrorHandler');

//运行
$app->bootstrap()->run();

