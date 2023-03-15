package dev_client

import (
	"context"
	"fmt"

	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/client-go/kubernetes"
	"k8s.io/utils/pointer"
)

// ClientSet: 是对 RESTClient 做了一层封装，把每一种资源都封装了对应的客户端。
//            把 Resource 和 Version 也封装成方法了，用起来更简单直接，一个资源是一个客户端，
//            多个资源就对应了多个客户端，所以 ClientSet 就是多个客户端的集合，
//            不过 ClientSet 只能访问内置资源，访问不了自定义资源
//
// 常用于对 k8s 内部资源做 CRUD, 或查询当前集群拥有什么资源

// 初始化客户端
func ClientSet() (*kubernetes.Clientset, error) {

	config, err := initConfig()
	if err != nil {
		panic(err)
	}
	// 实例化 clientSet 对象
	return kubernetes.NewForConfig(config)
}

// 创建 Namespace
func setCreateNamespace(clientSet *kubernetes.Clientset) {

	// 获取 namespace 的接口
	namespaceClient := clientSet.CoreV1().Namespaces()

	// 创建 namespace 所需的信息
	namespace := &corev1.Namespace{
		ObjectMeta: metav1.ObjectMeta{
			Name: NAMESPACE,
		},
		Status: corev1.NamespaceStatus{},
	}

	// 调用 Create 方法进行创建
	ns, err := namespaceClient.Create(context.TODO(), namespace, metav1.CreateOptions{})
	if err != nil {
		panic(err)
	}

	fmt.Println(ns.GetName(), ns)
}

// 创建 Deployment
func setCreateDeploy(clientSet *kubernetes.Clientset) {

	// 获取 deploy 的接口
	deployClient := clientSet.AppsV1().Deployments(NAMESPACE)

	// 定义要创建的 deployment 所需信息
	deploy := &appsv1.Deployment{
		ObjectMeta: metav1.ObjectMeta{
			Name:      DEPLOYMENT_TESTGO,
			Namespace: NAMESPACE,
			Labels: map[string]string{
				"app": "testgo",
			},
		},
		Spec: appsv1.DeploymentSpec{
			Replicas: pointer.Int32(2),
			Selector: &metav1.LabelSelector{
				MatchLabels: map[string]string{
					"app": "testgo",
				},
			},
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{
					Labels: map[string]string{
						"app": "testgo",
					},
				},
				Spec: corev1.PodSpec{
					Containers: []corev1.Container{
						{
							Name:  "testgo-container",
							Image: "registry.cn-shenzhen.aliyuncs.com/laixhe/testgo:v1",
							Ports: []corev1.ContainerPort{
								{
									ContainerPort: 80,
								},
							},
						},
					},
				},
			},
		},
		Status: appsv1.DeploymentStatus{},
	}

	// 调用 Create 方法进行创建
	result, err := deployClient.Create(context.TODO(), deploy, metav1.CreateOptions{})
	if err != nil {
		panic(err)
	}

	fmt.Println(result.GetName())
}

// 创建 Service
func setCreateService(clientSet *kubernetes.Clientset) {

	// 获取 serivce 的接口
	serviceClient := clientSet.CoreV1().Services(NAMESPACE)

	// 定义service
	service := &corev1.Service{
		ObjectMeta: metav1.ObjectMeta{
			Name:      SERVICE_TESTGO,
			Namespace: NAMESPACE,
		},
		Spec: corev1.ServiceSpec{
			Ports: []corev1.ServicePort{
				{
					Port:     80,
					NodePort: 30001,
				},
			},
			Selector: map[string]string{
				"app": "testgo",
			},
			Type: corev1.ServiceTypeNodePort,
		},
		Status: corev1.ServiceStatus{},
	}

	// 调用 Create 方法进行创建
	result, err := serviceClient.Create(context.TODO(), service, metav1.CreateOptions{})
	if err != nil {
		panic(err)
	}

	fmt.Println(result.GetName())
}

func setDeleteAll() {
	emptyDeleteOptions := metav1.DeleteOptions{}
	clientSet, err := ClientSet()
	if err != nil {
		return
	}
	// 删除 Service
	err = clientSet.CoreV1().Services(NAMESPACE).Delete(context.TODO(), SERVICE_TESTGO, emptyDeleteOptions)
	if err != nil {
		return
	}

	// 删除 Deployment
	err = clientSet.AppsV1().Deployments(NAMESPACE).Delete(context.TODO(), DEPLOYMENT_TESTGO, emptyDeleteOptions)
	if err != nil {
		return
	}

	// 删除 Namespace
	err = clientSet.CoreV1().Namespaces().Delete(context.TODO(), NAMESPACE, emptyDeleteOptions)
	if err != nil {
		return
	}

	// // 删除 Pod
	// err = clientSet.CoreV1().Pods("default").Delete(context.TODO(), "testgo", emptyDeleteOptions)
	// if err != nil {
	// 	return
	// }
}
