package dev_client

import (
	"context"
	"fmt"
	"log"

	corev1 "k8s.io/api/core/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/client-go/kubernetes/scheme"
	"k8s.io/client-go/rest"
)

// RESTClient: 这是最基础的客户端对象，仅对 HTTP Request 进行了封装，实现 RESTFul 风格 API，
//             这个对象的使用并不方便，因为很多参数都要使用者来设置

// 初始化客户端
func RESTClient() (*rest.RESTClient, error) {

	config, err := initConfig()
	if err != nil {
		panic(err)
	}

	// 因为 pod 的 group 为空，version 为 v1
	config.GroupVersion = &corev1.SchemeGroupVersion
	// 设置反序列化
	config.NegotiatedSerializer = scheme.Codecs
	// 指定 ApiPath,参考 /api/v1/namespaces/{namespace}/pods
	config.APIPath = "/api"

	// 根据配置文件，获取 rest 客户端
	return rest.RESTClientFor(config)

}

func restGetNamespaces(restClient *rest.RESTClient) {

	// 通过 rest client 获取 pod 的信息
	var podList corev1.PodList
	err := restClient.Get().
		Namespace(NAMESPACE).                                                    // 指定 namespace
		Resource("pods").                                                        // 指定要获取的资源对象
		VersionedParams(&metav1.ListOptions{Limit: 100}, scheme.ParameterCodec). // 指定大小限制和序列化工具
		Do(context.Background()).                                                // 执行
		Into(&podList)                                                           // 将结果写入 result
	if err != nil {
		log.Printf("get pods error:%v\n", err)
		return
	}

	// 获取到命名空间下的所有 Pod 的列表

	fmt.Println("namespace pod count:", len(podList.Items))
	for _, pod := range podList.Items {
		fmt.Printf("name: %s\n", pod.Name)
	}

}
