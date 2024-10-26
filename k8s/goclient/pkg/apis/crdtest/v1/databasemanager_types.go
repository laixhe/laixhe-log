package v1

import metav1 "k8s.io/apimachinery/pkg/apis/meta/v1"

// 定义资源类型

// +genclient
// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
// DatabaseManager 数据库管理
type DatabaseManager struct {
	metav1.TypeMeta   `json:",inline"`
	metav1.ObjectMeta `json:"metadata,omitempty"`

	Spec   DatabaseManagerSpec   `json:"spec"`
	Status DatabaseManagerStatus `json:"status"`
}

// DatabaseManagerSpec 期望状态
type DatabaseManagerSpec struct {
	DeploymentName string `json:"deploymentName"`
	Replicas       *int32 `json:"replicas"`
	DbType         string `json:"dbtype"`
}

// DatabaseManagerStatus 当前状态
type DatabaseManagerStatus struct {
	AvailableReplicas int32 `json:"availableReplicas"`
}

// +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object
// DatabaseManagerList 数据库管理列表
type DatabaseManagerList struct {
	metav1.TypeMeta `json:",inline"`
	metav1.ListMeta `json:"metadata"`

	Items []DatabaseManager `json:"items"`
}

/*
** +genclient表示需要创建client
** +k8s:deepcopy-gen:interfaces=k8s.io/apimachinery/pkg/runtime.Object表示在需要实现k8s.io/apimachinery/pkg/runtime.Object这个接口
**/
