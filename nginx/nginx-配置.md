
```
worker_processes  10;

worker_rlimit_nofile 65535;

events {
    worker_connections  10240;
}


http {
    include       mime.types;
    default_type  application/octet-stream;
    charset       utf-8;

    sendfile    on;
    tcp_nopush  on;
    tcp_nodelay on;
    keepalive_timeout     65;
    client_header_timeout 10;
    client_body_timeout   10;
    send_timeout 60;

    server_names_hash_bucket_size 128;
    client_max_body_size 1024m; 
    client_header_buffer_size 16k;
    large_client_header_buffers 4 32k;
    client_body_buffer_size 256k;

    gzip on;
    gzip_min_length 1k;
    gzip_buffers 4 16k;
    gzip_comp_level 4;
    gzip_types text/plain application/javascript application/x-javascript text/css application/xml text/javascript application/x-httpd-php image/jpeg image/gif image/png;

    proxy_connect_timeout 60;
    proxy_send_timeout 60;
    proxy_read_timeout 60;

    server {

        listen       40001;
        server_name  localhost;

        access_log  logs/access_40001.log;
        error_log   logs/error_40001.log;

        location / {
            proxy_pass http://127.0.0.1:40002;
            proxy_redirect off;
            proxy_set_header Host $http_host;
            proxy_set_header X-Forwarded-For $remote_addr;
            proxy_set_header X-Real-IP $remote_addr;
            proxy_set_header http_user_agent $http_user_agent;
        }


    }


}

```