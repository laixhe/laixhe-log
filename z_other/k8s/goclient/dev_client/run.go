package dev_client

import (
	"fmt"
)

const (
	NAMESPACE  = "dev"
	DEPLOYMENT = "dev-deploy"
	SERVICE    = "dev-service"

	DEPLOYMENT_TESTGO = DEPLOYMENT + "-testgo"
	SERVICE_TESTGO    = SERVICE + "-testgo"
)

func Run() {

	// 创建 RESTClient 的客户端
	restClient, err := RESTClient()
	if err != nil {
		panic(err)
	}

	// 通过 RESTClient 获取所有 namespace
	// restGetNamespaces(restClient)

	_ = restClient
	fmt.Println("-------- ClientSet --------")

	// 创建 ClientSet 的客户端
	clientSet, err := ClientSet()
	if err != nil {
		panic(err)
	}

	// 通过 clientSet 创建 namespace
	// setCreateNamespace(clientSet)
	// 通过 clientSet 创建 deployment
	// setCreateDeploy(clientSet)
	// 通过 clientSet 创建 service
	// setCreateService(clientSet)

	_ = clientSet
	fmt.Println("-------- DynamicClient --------")

	// 创建 DynamicClient 的客户端
	dynamicClient, err := DynamicClient()
	if err != nil {
		panic(err)
	}

	// 通过 DynamicClient 创建 deployment
	// dynamicCreateDeploy(dynamicClient)
	dynamicListDeploy(dynamicClient)

	_ = dynamicClient
}
