package dev_client

import (
	"k8s.io/client-go/rest"
	"k8s.io/client-go/tools/clientcmd"
)

// 获取 config，从本机中获取 kubeconfig 的配置文件 ( $HOME/.kube/config )
func initConfig() (*rest.Config, error) {

	// 从本机中获取 kubeconfig 的配置文件，因此第一个参数是空字符串
	config, err := clientcmd.BuildConfigFromFlags("", clientcmd.RecommendedHomeFile)
	if err != nil {
		return nil, err
	}
	return config, nil
}
