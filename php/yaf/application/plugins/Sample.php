<?php

use Yaf\Plugin_Abstract;
use Yaf\Request_Abstract;
use Yaf\Response_Abstract;

/**
 * @name SamplePlugin
 * @desc Yaf定义了如下的6个Hook,插件之间的执行顺序是先进先Call
 * @see http://www.php.net/manual/en/class.yaf-plugin-abstract.php
 * @author root
 */
class SamplePlugin extends Plugin_Abstract {

    /* 在路由之前执行,这个钩子里，你可以做url重写等功能 */
    public function routerStartup(Request_Abstract $request, Response_Abstract $response) {
    }

    /* 路由完成后，在这个钩子里，你可以做登陆检测等功能*/
    public function routerShutdown(Request_Abstract $request, Response_Abstract $response) {
    }

    /* 循环调度开始*/
    public function dispatchLoopStartup(Request_Abstract $request, Response_Abstract $response) {
    }

    /* 单个循环调度开始*/
    public function preDispatch(Request_Abstract $request, Response_Abstract $response) {
    }

    /* 单个循环调度结束*/
    public function postDispatch(Request_Abstract $request, Response_Abstract $response) {
    }

    /* 循环调度结束*/
    public function dispatchLoopShutdown(Request_Abstract $request, Response_Abstract $response) {
    }

}
