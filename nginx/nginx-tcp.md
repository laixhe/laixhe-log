```
stream {

	# upstream
	
	server{
		listen     12345; #listen 12345 udp;
		proxy_pass 39.108.125.199:12345;
	}
	
	server{
		listen     5222;
		proxy_pass 39.108.125.199:5222;
	}
	
}
```