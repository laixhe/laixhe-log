package exec_main

import (
	"bytes"
	"log"
	"os/exec"
	"strings"
)

// Netstat 执行终端命令
func Netstat() {

	cmd := exec.Command("netstat", "-ntlp")

	var out bytes.Buffer
	cmd.Stdout = &out

	// 运行命令
	if err := cmd.Run(); err != nil {
		log.Println("cmd.Run", err)
		return
	}

	str := out.String()

	if !strings.Contains(str, "nginx") {

		zimg := exec.Command("/usr/local/nginx/sbin/nginx", "-c", "/usr/local/nginx/conf/nginx.conf")
		_, err := zimg.Output()
		if err != nil {
			log.Println("nginx.Output", err)
			return
		}

	}

}
