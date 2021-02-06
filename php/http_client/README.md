### http 客户端

##### CURL  请求
```
<?php

// 初始化CURL句柄
$curl = curl_init();

curl_setopt_array($curl, array(
  CURLOPT_URL => "http://127.0.0.1:5050",    // 设置请求的URL
  CURLOPT_RETURNTRANSFER => true,            // 设为 TRUE 把 curl_exec() 结果转化为字串，而不是直接输出
  CURLOPT_ENCODING => "",
  CURLOPT_MAXREDIRS => 10,
  CURLOPT_TIMEOUT => 0,                      // 超时设置,以秒为单位
  CURLOPT_FOLLOWLOCATION => true,
  CURLOPT_HTTP_VERSION => CURL_HTTP_VERSION_1_1,
  CURLOPT_CUSTOMREQUEST => "POST",           // 设置请求方式
  CURLOPT_POSTFIELDS => "id=1&name=laixhe",  // 设置提交的字符串
  CURLOPT_HTTPHEADER => array(
    "Content-Type: application/x-www-form-urlencoded" // 设置请求头
  ),
));

$response = curl_exec($curl);

curl_close($curl);
echo $response;
?>
```