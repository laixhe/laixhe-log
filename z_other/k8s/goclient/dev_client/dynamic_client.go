package dev_client

import (
	"context"
	"fmt"

	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/apis/meta/v1/unstructured"
	"k8s.io/apimachinery/pkg/runtime/schema"
	"k8s.io/client-go/dynamic"
)

// DynamicClient: 可以访问内置资源和CRD自定义资源，拿出的内容是 Object 类型，按实际情况自己去做强制转换，当然了也会有强转失败的风险
// 如果只是操作 kubernetes 的内置资源，用 RESTClient、ClientSet 已经足够了，但是对于 CRD 资源却无能为力了

func DynamicClient() (*dynamic.DynamicClient, error) {

	config, err := initConfig()
	if err != nil {
		panic(err)
	}
	// 创建 dynamic 客户端
	return dynamic.NewForConfig(config)
}

func dynamicCreateDeploy(dynamicClient *dynamic.DynamicClient) {

	deploymentRes := schema.GroupVersionResource{
		Group:    "apps",
		Version:  "v1",
		Resource: "deployments",
	}

	deployment := &unstructured.Unstructured{

		Object: map[string]interface{}{
			"apiVersion": "apps/v1",
			"kind":       "Deployment",
			"metadata": map[string]interface{}{
				"name":      DEPLOYMENT_TESTGO,
				"namespace": "dev",
			},
			"spec": map[string]interface{}{
				"replicas": 2,
				"selector": map[string]interface{}{
					"matchLabels": map[string]interface{}{
						"app": "testgo",
					},
				},
				"template": map[string]interface{}{
					"metadata": map[string]interface{}{
						"labels": map[string]interface{}{
							"app": "testgo",
						},
					},
					"spec": map[string]interface{}{
						"containers": []map[string]interface{}{
							{
								"name":  "testgo-container",
								"image": "registry.cn-shenzhen.aliyuncs.com/laixhe/testgo:v1",
								"ports": []map[string]interface{}{
									{
										"containerPort": 80,
									},
								},
							},
						},
					},
				},
			},
		},
	}

	// 创建 Deployment
	result, err := dynamicClient.
		Resource(deploymentRes).
		Namespace(NAMESPACE).
		Create(context.TODO(), deployment, metav1.CreateOptions{})
	if err != nil {
		panic(err)
	}

	fmt.Printf("Create deployment %q.\n", result.GetName())

}

func dynamicListDeploy(dynamicClient *dynamic.DynamicClient) {

	deploymentRes := schema.GroupVersionResource{
		Group:    "apps",
		Version:  "v1",
		Resource: "deployments",
	}

	list, err := dynamicClient.Resource(deploymentRes).Namespace(NAMESPACE).List(context.TODO(), metav1.ListOptions{})
	if err != nil {
		panic(err)
	}

	for _, d := range list.Items {

		replicas, found, err := unstructured.NestedInt64(d.Object, "spec", "replicas")
		if err != nil || !found {
			fmt.Printf("Replicas not found for deployment %s: error=%s", d.GetName(), err)
			continue
		}

		fmt.Printf("%s %s(%d replicas)\n", d.GetNamespace(), d.GetName(), replicas)
	}
}
