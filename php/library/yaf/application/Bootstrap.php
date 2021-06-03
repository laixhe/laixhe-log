<?php

use Yaf\Loader;
use Yaf\Bootstrap_Abstract;
use Yaf\Application;
use Yaf\Dispatcher;
use Yaf\Registry;
use Yaf\Route\Regex;

/**
 * 所有在Bootstrap类中, 以_init开头的方法, 都会被Yaf调用,
 * 这些方法, 都接受一个参数:Yaf_Dispatcher $dispatcher
 * 调用的次序, 和申明的次序相同
 */
class Bootstrap extends Bootstrap_Abstract{

    //public function _initLoader(){
    //
    //    // 加载vendor下的文件
    //    Loader::import(APPLICATION_PATH . '/vendor/autoload.php');
    //
    //}

    public function _initConfig(Dispatcher $dispatcher) {

        // 获取配置信息并保存起来
        $arrConfig = Application::app()->getConfig();
        Registry::set('config', $arrConfig);

    }

    public function _initRoute(Dispatcher $dispatcher) {

        //在这里注册自己的路由协议,默认使用简单路由

        //使用定义简单路由simple
        //$router = $dispatcher->getInstance()->getRouter();
        //$route = new Yaf\Route\Simple('m', 'c', 'a');
        //$router->addRoute('simple',$route);

        // 初始化路由
        $router = $dispatcher->getInstance()->getRouter();

        // 初始化配置路由
        $router->addConfig(Registry::get('config')->routes);

        // 可通过代码添加路由
        $v10_index_index_test = new Regex(
            '#/v1.0/test_test/([0-9]+)#', // 必须要用定界符（本例子为"#")，否则报错

            [
                'module'=>'Index',
                'controller' => 'Index',
                'action' => 'test'
            ],

            ["1"=>'id']
        );

        $router->addRoute('product', $v10_index_index_test);
        $router->addRoute('develop', $v10_index_index_test);

    }

    public function _initSeasLog(Dispatcher $dispatcher){

        // 初始化 SeasLog 日志

        $basepath = Registry::get('config')->seasLog->basepath;
        $logger = Registry::get('config')->seasLog->logger;

        Seaslog::setBasePath($basepath);
        SeasLog::setLogger($logger);

    }

    //public function _initPlugin(Yaf\Dispatcher $dispatcher) {
        //注册一个插件
        //$objSamplePlugin = new SamplePlugin();
        //$dispatcher->registerPlugin($objSamplePlugin);
    //}

    //public function _initView(Yaf\Dispatcher $dispatcher){
        //在这里注册自己的view控制器，例如smarty,firekylin

        //禁用view
        //$dispatcher->getInstance()->disableView();
    //}

    //public function _initSession(Yaf\Dispatcher $dispatcher){
        //在这里注册session
    //    session_id() || session_start();
    //}

}
