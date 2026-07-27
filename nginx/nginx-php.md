```

worker_processes  2;

worker_rlimit_nofile 65535;

events {
    worker_connections  10240;
}


http {
    include       mime.types;
    default_type  application/octet-stream;

    sendfile       on;
    tcp_nopush     on;

    keepalive_timeout  65;

    gzip  on;

    server {
        listen       81;
        server_name  localhost;

        charset utf-8;
        
        root   "D:/Code/php/php-yaf/public";
        index  index.php index.html index.htm;

        location / {
            # 判断是否是目录或文件，不是就重写 url
            if (!-e $request_filename) {
                rewrite ^(.*)$ /index.php/$1 last;
                break;
            }
        }
        
        location ~ \.php(.*)$  {
            fastcgi_pass     127.0.0.1:9000;
            fastcgi_index    index.php;
            fastcgi_param    SCRIPT_FILENAME  $document_root$fastcgi_script_name;
            fastcgi_param    PATH_INFO $1;
            include          fastcgi_params;
        }
        
        #location / {
        #     try_files $uri $uri/ /index.php?$query_string;
        #}
        
        #location ~ \.php$ {
        #    fastcgi_pass 127.0.0.1:9000;
        #    fastcgi_param SCRIPT_FILENAME $realpath_root$fastcgi_script_name;
        #    include fastcgi_params;
        #}
        

    }

}

```