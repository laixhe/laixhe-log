<?php

use Yaf\Bootstrap_Abstract;

/**
 * 当有未捕获的异常, 则控制流会流到这里
 */
class ErrorController extends Controller_Abstract {

    public function errorAction($exception) {
        echo "Error Msg:" . $exception->getMessage();
        echo "Error Code:" . $exception->getCode();
        return true;
    }

}