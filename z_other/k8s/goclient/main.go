package main

import (
	"flag"
	"time"

	kubeinformers "k8s.io/client-go/informers"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/tools/clientcmd"
	"k8s.io/klog/v2"

	"goclient/pkg/controller/databasemanager"
	clientset "goclient/pkg/generated/clientset/versioned"
	informers "goclient/pkg/generated/informers/externalversions"
	"goclient/pkg/signals"
)

var (
	masterURL  string
	kubeconfig string
)

func main() {
	// klog.InitFlags(nil)
	flag.Parse()
	// 设置处理系统信号的Channel
	stopCh := signals.SetupSignalHandler()
	// 处理入参
	cfg, err := clientcmd.BuildConfigFromFlags(masterURL, kubeconfig)
	if err != nil {
		klog.Fatalf("Error building kubeconfig: %s", err.Error())
	}
	// 初始化kubeClient
	kubeClient, err := kubernetes.NewForConfig(cfg)
	if err != nil {
		klog.Fatalf("Error building kubernetes clientset: %s", err.Error())
	}
	// 初始化dbmanagerClient
	dbmanagerClient, err := clientset.NewForConfig(cfg)
	if err != nil {
		klog.Fatalf("Error building example clientset: %s", err.Error())
	}
	kubeInformerFactory := kubeinformers.NewSharedInformerFactory(kubeClient, time.Second*30)
	dbmanagerInformerFactory := informers.NewSharedInformerFactory(dbmanagerClient, time.Second*30)
	// 初始化controller
	controller := databasemanager.NewController(kubeClient, dbmanagerClient,
		dbmanagerInformerFactory.Crdtest().V1().DatabaseManagers(), kubeInformerFactory.Apps().V1().Deployments())
	// notice that there is no need to run Start methods in a separate goroutine. (i.e. go kubeInformerFactory.Start(stopCh)
	// Start method is non-blocking and runs all registered informers in a dedicated goroutine.
	kubeInformerFactory.Start(stopCh)
	dbmanagerInformerFactory.Start(stopCh)
	if err = controller.Run(2, stopCh); err != nil {
		klog.Fatalf("Error running controller: %s", err.Error())
	}
}
func init() {
	flag.StringVar(&kubeconfig, "kubeconfig", "", "Path to a kubeconfig. Only required if out-of-cluster.")
	flag.StringVar(&masterURL, "master", "", "The address of the Kubernetes API server. Overrides any value in kubeconfig. Only required if out-of-cluster.")
}
