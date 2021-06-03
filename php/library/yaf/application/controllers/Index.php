<?php

use Yaf\Controller_Abstract;
use Yaf\Registry;

/**
 * 主页控制器 
 */
class IndexController extends Controller_Abstract {

    /**
     * 先执行(可以用于初始化，类似构造函方法)
     */
    public function init(){

        // 获取 yaf 运行环境
        //echo ini_get('yaf.environ');
        //echo Yaf\Application::app()->environ();
    }


    public function indexAction(){
        
        // 调用test模型
        $getdata = TestModel::getTest();

        // 将变量输出到视图
        $this->getView()->assign('getdata', $getdata);

        // 是否要输出视图
        return true;
    }

    public function configAction(){

        var_dump(Registry::get('config'));

        return false;
    }

    public function testAction(){

        $paras = $this->getRequest()->getParams();

        var_dump($paras);
        var_dump(__METHOD__);

        return false;
    }

}
