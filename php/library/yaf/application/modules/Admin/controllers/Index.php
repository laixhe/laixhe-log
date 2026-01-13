<?php

use Yaf\Controller_Abstract;
/**
 * 后台管理首页
 */
class IndexController extends Controller_Abstract{
    /**
     * 先执行(可以用于初始化，类似构造函方法)
     */
    public function init(){
       
    }

    /**
     * 主页
     */
    public function indexAction(){
        
        //引入头部公共文件
        $headhtml = $this->getView()->render('public/head.php');
        $this->getView()->assign('headhtml', $headhtml);
        return true;
    }
}