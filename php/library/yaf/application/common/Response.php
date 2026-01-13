<?php

define("CODE_SUCCESS", 0);
define("CODE_ERROR_UNKNOWN", 1);         // Unknown Error
define("CODE_ERROR_UNSUPPORT_API", 2);   // UnSupport API
define("CODE_ERROR_PARAMETER_EMPTY", 3); // parameter Empty 参数为空
define("CODE_ERROR_PARAMETER_ERROR", 4); // parameter Error 参数错误
define("CODE_ERROR_RESOURCES_ERROR", 5); // resources Error 资源错误
define("CODE_ERROR_INVALID_JSON", 6);    // Invalid JSON Format
define("CODE_ERROR_ENCODE_JSON", 7);     // Encode JSON Error


define("DATA_CODE", [
    CODE_ERROR_UNKNOWN => "Unknown Error",
    CODE_ERROR_UNSUPPORT_API => "UnSupport API",
    CODE_ERROR_PARAMETER_EMPTY => "Parameter Empty",
    CODE_ERROR_PARAMETER_ERROR => "Parameter Error",
    CODE_ERROR_RESOURCES_ERROR => "Resources Error",
    CODE_ERROR_INVALID_JSON => "Invalid JSON Format",
    CODE_ERROR_ENCODE_JSON => "Encode JSON Error",
]);


class Response {
    public $code = 0;
    public $message = "";
    public $data = null;

    public function __construct(int $code, string $message, $data=null){
        $this->code = $code;
        $this->message = $message;
        $this->data = $data;
    }
}

function jsonResponse($data) : string {

    $dataJson = json_encode(new Response(CODE_SUCCESS, "", $data), JSON_UNESCAPED_UNICODE);
    if ($dataJson) {
        return $dataJson;
    }

    return jsonResponseErr(CODE_ERROR_INVALID_JSON);
}

function jsonResponseErr(int $code, string $message = "") : string {

    if(empty($message)){

        if (isset(DATA_CODE[$code])){
            $message = DATA_CODE[$code];
        }

    }

    return json_encode(new Response($code, $message), JSON_UNESCAPED_UNICODE);
}

function isRequestMethod(bool $is){

    if (!$is) {
        echo jsonResponseErr(CODE_ERROR_UNSUPPORT_API);
        exit(1);
    }

}

function isRequestJson() : bool {

    $content = $_SERVER['CONTENT_TYPE'];
    $content = trim($content);
    if('application/json' == $content){
        return true;
    }

    return false;
}

function getRequestJson() : array {

    if(isRequestJson()){
        $jsonStr = file_get_contents("php://input");
        if(!empty($jsonStr)){
            $jsonData = json_decode($jsonStr, true);
            if(is_array($jsonData)){
                return $jsonData;
            }
        }
    }

    return [];

}

function getPost(string $name, $default) {

    static $jsonData = [];
    static $isJson = false;
    static $isData = false;

    if (!$isData) {

        $jsonData = getRequestJson();

        if (!empty($jsonData)) {
            $isJson = true;
        }

        $isData = true;

    }

    if($isJson){

        if(isset($jsonData[$name])){
            if (is_string($jsonData[$name])){
                return trim($jsonData[$name]);
            }

            return $jsonData[$name];
        }

    } else {

        if(isset($_POST[$name])){
            return trim($_POST[$name]);
        }

        if(isset($_GET[$name])){
            return trim($_GET[$name]);
        }

    }

    return $default;

}
