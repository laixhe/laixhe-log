// Code generated by client-gen. DO NOT EDIT.

package fake

import (
	v1 "goclient/pkg/generated/clientset/versioned/typed/crdtest/v1"

	rest "k8s.io/client-go/rest"
	testing "k8s.io/client-go/testing"
)

type FakeCrdtestV1 struct {
	*testing.Fake
}

func (c *FakeCrdtestV1) DatabaseManagers(namespace string) v1.DatabaseManagerInterface {
	return &FakeDatabaseManagers{c, namespace}
}

// RESTClient returns a RESTClient that is used to communicate
// with API server by this client implementation.
func (c *FakeCrdtestV1) RESTClient() rest.Interface {
	var ret *rest.RESTClient
	return ret
}
