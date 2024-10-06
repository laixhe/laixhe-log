package databasemanager

import (
	"context"
	"fmt"
	"time"

	"github.com/golang/glog"
	appsv1 "k8s.io/api/apps/v1"
	corev1 "k8s.io/api/core/v1"
	"k8s.io/apimachinery/pkg/api/errors"
	metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	"k8s.io/apimachinery/pkg/util/runtime"
	utilruntime "k8s.io/apimachinery/pkg/util/runtime"
	"k8s.io/apimachinery/pkg/util/wait"
	appsinformers "k8s.io/client-go/informers/apps/v1"
	"k8s.io/client-go/kubernetes"
	"k8s.io/client-go/kubernetes/scheme"
	typedcorev1 "k8s.io/client-go/kubernetes/typed/core/v1"
	appslisters "k8s.io/client-go/listers/apps/v1"
	"k8s.io/client-go/tools/cache"
	"k8s.io/client-go/tools/record"
	"k8s.io/client-go/util/workqueue"
	"k8s.io/klog/v2"

	dbmanagerv1 "goclient/pkg/apis/crdtest/v1"
	clientset "goclient/pkg/generated/clientset/versioned"
	dbmanagerscheme "goclient/pkg/generated/clientset/versioned/scheme"
	informers "goclient/pkg/generated/informers/externalversions/crdtest/v1"
	listers "goclient/pkg/generated/listers/crdtest/v1"
)

const controllerAgentName = "database-manager-controller"

const (
	// SuccessSynced 用来表示事件被成功同步
	SuccessSynced = "Synced"
	// MessageResourceSynced 表示事件被触发时的消息信息
	MessageResourceSynced = "database manager synced successfully"
	MessageResourceExists = "Resource %q already exists and is not managed by DatabaseManager"
	ErrResourceExists     = "ErrResourceExists"
)

type Controller struct {
	// kubeclientset 是kubernetes的clientset
	kubeclientset kubernetes.Interface
	// dbmanagerclientset 是自己定义的API Group的clientset
	dbmanagerclientset clientset.Interface

	// deploymentsLister list deployment 对象
	deploymentsLister appslisters.DeploymentLister
	// deploymentsSynced 同步deployment对象
	deploymentsSynced cache.InformerSynced

	// dbmanagerLister list databasemanager 对象
	dbmanagerLister listers.DatabaseManagerLister
	// dbmanagerSynced 同步 DatabaseManager 对象
	dbmanagerSynced cache.InformerSynced

	// workqueue 限速的队列
	workqueue workqueue.RateLimitingInterface
	// recorder 事件记录器
	recorder record.EventRecorder
}

// NewController 初始化Controller
func NewController(kubeclientset kubernetes.Interface, dbmanagerclientset clientset.Interface,
	dbmanagerinformer informers.DatabaseManagerInformer, deploymentInformer appsinformers.DeploymentInformer) *Controller {

	utilruntime.Must(dbmanagerscheme.AddToScheme(scheme.Scheme))
	glog.V(4).Info("Create event broadcaster")
	// 创建eventBroadcaster
	eventBroadcaster := record.NewBroadcaster()
	// 保存events到日志
	eventBroadcaster.StartLogging(glog.Infof)
	// 上报events到APIServer
	eventBroadcaster.StartRecordingToSink(&typedcorev1.EventSinkImpl{Interface: kubeclientset.CoreV1().Events("")})
	recorder := eventBroadcaster.NewRecorder(scheme.Scheme, corev1.EventSource{Component: controllerAgentName})

	// 初始化Controller
	controller := &Controller{
		kubeclientset:      kubeclientset,
		dbmanagerclientset: dbmanagerclientset,
		deploymentsLister:  deploymentInformer.Lister(),
		deploymentsSynced:  deploymentInformer.Informer().HasSynced,
		dbmanagerLister:    dbmanagerinformer.Lister(),
		dbmanagerSynced:    dbmanagerinformer.Informer().HasSynced,
		workqueue:          workqueue.NewNamedRateLimitingQueue(workqueue.DefaultControllerRateLimiter(), "DatabaseManagers"),
		recorder:           recorder,
	}

	glog.Info("Start up event handlers")

	// 注册Event Handler,分别对于添加、更新、删除事件，具体的操作由事件对应的API将其加入队列中
	dbmanagerinformer.Informer().AddEventHandler(cache.ResourceEventHandlerFuncs{
		AddFunc: controller.enqueueDatabaseManager,
		UpdateFunc: func(oldObj, newObj interface{}) {
			oldDBManager := oldObj.(*dbmanagerv1.DatabaseManager)
			newDBManager := newObj.(*dbmanagerv1.DatabaseManager)
			if oldDBManager.ResourceVersion == newDBManager.ResourceVersion {
				return
			}
			controller.enqueueDatabaseManager(newObj)
		},
		DeleteFunc: controller.enqueueDatabaseManagerForDelete,
	})

	// 注册Deployment Event Handler
	deploymentInformer.Informer().AddEventHandler(cache.ResourceEventHandlerFuncs{
		AddFunc: controller.handleObject,
		UpdateFunc: func(old, new interface{}) {
			newDepl := new.(*appsv1.Deployment)
			oldDepl := old.(*appsv1.Deployment)
			if newDepl.ResourceVersion == oldDepl.ResourceVersion {
				// 如果没有改变，就返回
				return
			}
			controller.handleObject(new)
		},
		DeleteFunc: controller.handleObject,
	})

	return controller
}

// Run 启动入口
func (c *Controller) Run(threadiness int, stopCh <-chan struct{}) error {
	defer utilruntime.HandleCrash()
	defer c.workqueue.ShuttingDown()

	glog.Info("start controller, cache sync")
	// 同步缓存数据
	if ok := cache.WaitForCacheSync(stopCh, c.dbmanagerSynced); !ok {
		return fmt.Errorf("failed to wait for caches to sync")
	}

	glog.Info("begin start worker thread")
	// 开启work线程
	for i := 0; i < threadiness; i++ {
		go wait.Until(c.runWorker, time.Second, stopCh)
	}

	glog.Info("worker thread started!!!!!!")
	<-stopCh
	glog.Info("worker thread stopped!!!!!!")
	return nil
}

// runWorker 是一个死循环，会一直调用processNextWorkItem从workqueue中取出数据
func (c *Controller) runWorker() {
	for c.processNextWorkItem() {
	}
}

// processNextWorkItem 从workqueue中取出数据进行处理
func (c *Controller) processNextWorkItem() bool {
	obj, shutdown := c.workqueue.Get()

	if shutdown {
		return false
	}

	// We wrap this block in a func so we can defer c.workqueue.Done.
	err := func(obj interface{}) error {
		defer c.workqueue.Done(obj)
		var key string
		var ok bool

		if key, ok = obj.(string); !ok {
			c.workqueue.Forget(obj)
			runtime.HandleError(fmt.Errorf("expected string in workqueue but got %#v", obj))
			return nil
		}
		// 在syncHandler中处理业务
		if err := c.syncHandler(key); err != nil {
			return fmt.Errorf("error syncing '%s': %s", key, err.Error())
		}

		c.workqueue.Forget(obj)
		glog.Infof("Successfully synced '%s'", key)
		return nil
	}(obj)

	if err != nil {
		runtime.HandleError(err)
		return true
	}

	return true
}

// syncHandler 处理业务Handler
func (c *Controller) syncHandler(key string) error {
	// 通过split得到namespace和name
	namespace, name, err := cache.SplitMetaNamespaceKey(key)
	if err != nil {
		runtime.HandleError(fmt.Errorf("invalid resource key: %s", key))
		return nil
	}

	// 从缓存中取对象
	dbManager, err := c.dbmanagerLister.DatabaseManagers(namespace).Get(name)
	if err != nil {
		// 如果DatabaseManager对象被删除了，就会走到这里
		if errors.IsNotFound(err) {
			glog.Infof("DatabaseManager对象被删除，请在这里执行实际的删除业务: %s/%s ...", namespace, name)
			return nil
		}

		runtime.HandleError(fmt.Errorf("failed to list DatabaseManager by: %s/%s", namespace, name))

		return err
	}

	glog.Infof("这里是databasemanager对象的期望状态: %#v ...", dbManager)

	// 获取是否有deploymentName
	deploymentName := dbManager.Spec.DeploymentName

	if deploymentName == "" {
		utilruntime.HandleError(fmt.Errorf("%s: deploymentName 不能为空", key))
		return nil
	}
	// 判断deployment是否在集群中存在
	deployment, err := c.deploymentsLister.Deployments(dbManager.Namespace).Get(deploymentName)
	if errors.IsNotFound(err) {
		// 如果没有找到，就创建
		deployment, err = c.kubeclientset.AppsV1().Deployments(dbManager.Namespace).Create(
			context.TODO(), newDeployment(dbManager), metav1.CreateOptions{})
	}

	// 如果Create 或者 Get 都出错，则返回
	if err != nil {
		return err
	}

	// 如果这个deployment不是由DatabaseManager控制，应该报告这个事件
	if !metav1.IsControlledBy(deployment, dbManager) {
		msg := fmt.Sprintf(MessageResourceExists, deployment.Name)
		c.recorder.Event(dbManager, corev1.EventTypeWarning, ErrResourceExists, msg)
		return fmt.Errorf("%s", msg)
	}

	// 如果replicas和期望的不等，则更新deployment
	if dbManager.Spec.Replicas != nil && *dbManager.Spec.Replicas != *deployment.Spec.Replicas {
		klog.V(4).Infof("DatabaseManager %s replicas: %d, deployment replicas: %d", name, *dbManager.Spec.Replicas, *deployment.Spec.Replicas)
		deployment, err = c.kubeclientset.AppsV1().Deployments(dbManager.Namespace).Update(context.TODO(), newDeployment(dbManager), metav1.UpdateOptions{})
	}

	if err != nil {
		return err
	}

	// 更新状态
	err = c.updateDatabaseManagerStatus(dbManager, deployment)
	if err != nil {
		return err
	}

	glog.Infof("实际状态是从业务层面得到的，此处应该去的实际状态，与期望状态做对比，并根据差异做出响应(新增或者删除)")

	c.recorder.Event(dbManager, corev1.EventTypeNormal, SuccessSynced, MessageResourceSynced)
	return nil
}

// updateDatabaseManagerStatus 更新DatabaseManager状态
func (c *Controller) updateDatabaseManagerStatus(dbmanager *dbmanagerv1.DatabaseManager, deployment *appsv1.Deployment) error {
	dbmanagerCopy := dbmanager.DeepCopy()
	dbmanagerCopy.Status.AvailableReplicas = deployment.Status.AvailableReplicas
	_, err := c.dbmanagerclientset.CrdtestV1().DatabaseManagers(dbmanager.Namespace).Update(context.TODO(), dbmanagerCopy, metav1.UpdateOptions{})
	return err
}

func (c *Controller) handleObject(obj interface{}) {
	var object metav1.Object
	var ok bool
	if object, ok = obj.(metav1.Object); !ok {
		tombstone, ok := obj.(cache.DeletedFinalStateUnknown)
		if !ok {
			utilruntime.HandleError(fmt.Errorf("error decoding object, invalid type"))
			return
		}
		object, ok = tombstone.Obj.(metav1.Object)
		if !ok {
			utilruntime.HandleError(fmt.Errorf("error decoding object tombstone, invalid type"))
			return
		}
		klog.V(4).Infof("Recovered deleted object '%s' from tombstone", object.GetName())
	}
	klog.V(4).Infof("Processing object: %s", object.GetName())
	if ownerRef := metav1.GetControllerOf(object); ownerRef != nil {
		// 检查对象是否和DatabaseManager对象关联，如果不是就退出
		if ownerRef.Kind != "DatabaseManager" {
			return
		}

		dbmanage, err := c.dbmanagerLister.DatabaseManagers(object.GetNamespace()).Get(ownerRef.Name)
		if err != nil {
			klog.V(4).Infof("ignoring orphaned object '%s' of databaseManager '%s'", object.GetSelfLink(), ownerRef.Name)
			return
		}

		c.enqueueDatabaseManager(dbmanage)
		return
	}
}

func newDeployment(dbmanager *dbmanagerv1.DatabaseManager) *appsv1.Deployment {
	var image string
	var name string
	switch dbmanager.Spec.DbType {
	case "mysql":
		image = "mysql:5.7"
		name = "mysql"
	case "mariadb":
		image = "mariadb:10.7.1"
		name = "mariadb"
	default:
		image = "mysql:5.7"
		name = "mysql"
	}

	labels := map[string]string{
		"app": dbmanager.Spec.DbType,
	}
	return &appsv1.Deployment{
		ObjectMeta: metav1.ObjectMeta{
			Namespace: dbmanager.Namespace,
			Name:      dbmanager.Name,
			OwnerReferences: []metav1.OwnerReference{
				*metav1.NewControllerRef(dbmanager, dbmanagerv1.SchemeGroupVersion.WithKind("DatabaseManager")),
			},
		},
		Spec: appsv1.DeploymentSpec{
			Replicas: dbmanager.Spec.Replicas,
			Selector: &metav1.LabelSelector{MatchLabels: labels},
			Template: corev1.PodTemplateSpec{
				ObjectMeta: metav1.ObjectMeta{Labels: labels},
				Spec: corev1.PodSpec{
					Containers: []corev1.Container{
						{
							Name:  name,
							Image: image,
						},
					},
				},
			},
		},
	}
}

// 数据先放入缓存，再入队列
func (c *Controller) enqueueDatabaseManager(obj interface{}) {
	var key string
	var err error
	// 将对象放入缓存
	if key, err = cache.MetaNamespaceKeyFunc(obj); err != nil {
		runtime.HandleError(err)
		return
	}

	// 将key放入队列
	c.workqueue.AddRateLimited(key)
}

// 删除操作
func (c *Controller) enqueueDatabaseManagerForDelete(obj interface{}) {
	var key string
	var err error
	// 从缓存中删除指定对象
	key, err = cache.DeletionHandlingMetaNamespaceKeyFunc(obj)
	if err != nil {
		runtime.HandleError(err)
		return
	}
	//再将key放入队列
	c.workqueue.AddRateLimited(key)
}
