// Code generated by client-gen. DO NOT EDIT.

package fake

import (
	"context"
	crdtestv1 "goclient/pkg/apis/crdtest/v1"

	v1 "k8s.io/apimachinery/pkg/apis/meta/v1"
	labels "k8s.io/apimachinery/pkg/labels"
	schema "k8s.io/apimachinery/pkg/runtime/schema"
	types "k8s.io/apimachinery/pkg/types"
	watch "k8s.io/apimachinery/pkg/watch"
	testing "k8s.io/client-go/testing"
)

// FakeDatabaseManagers implements DatabaseManagerInterface
type FakeDatabaseManagers struct {
	Fake *FakeCrdtestV1
	ns   string
}

var databasemanagersResource = schema.GroupVersionResource{Group: "crdtest.laixhe.com", Version: "v1", Resource: "databasemanagers"}

var databasemanagersKind = schema.GroupVersionKind{Group: "crdtest.laixhe.com", Version: "v1", Kind: "DatabaseManager"}

// Get takes name of the databaseManager, and returns the corresponding databaseManager object, and an error if there is any.
func (c *FakeDatabaseManagers) Get(ctx context.Context, name string, options v1.GetOptions) (result *crdtestv1.DatabaseManager, err error) {
	obj, err := c.Fake.
		Invokes(testing.NewGetAction(databasemanagersResource, c.ns, name), &crdtestv1.DatabaseManager{})

	if obj == nil {
		return nil, err
	}
	return obj.(*crdtestv1.DatabaseManager), err
}

// List takes label and field selectors, and returns the list of DatabaseManagers that match those selectors.
func (c *FakeDatabaseManagers) List(ctx context.Context, opts v1.ListOptions) (result *crdtestv1.DatabaseManagerList, err error) {
	obj, err := c.Fake.
		Invokes(testing.NewListAction(databasemanagersResource, databasemanagersKind, c.ns, opts), &crdtestv1.DatabaseManagerList{})

	if obj == nil {
		return nil, err
	}

	label, _, _ := testing.ExtractFromListOptions(opts)
	if label == nil {
		label = labels.Everything()
	}
	list := &crdtestv1.DatabaseManagerList{ListMeta: obj.(*crdtestv1.DatabaseManagerList).ListMeta}
	for _, item := range obj.(*crdtestv1.DatabaseManagerList).Items {
		if label.Matches(labels.Set(item.Labels)) {
			list.Items = append(list.Items, item)
		}
	}
	return list, err
}

// Watch returns a watch.Interface that watches the requested databaseManagers.
func (c *FakeDatabaseManagers) Watch(ctx context.Context, opts v1.ListOptions) (watch.Interface, error) {
	return c.Fake.
		InvokesWatch(testing.NewWatchAction(databasemanagersResource, c.ns, opts))

}

// Create takes the representation of a databaseManager and creates it.  Returns the server's representation of the databaseManager, and an error, if there is any.
func (c *FakeDatabaseManagers) Create(ctx context.Context, databaseManager *crdtestv1.DatabaseManager, opts v1.CreateOptions) (result *crdtestv1.DatabaseManager, err error) {
	obj, err := c.Fake.
		Invokes(testing.NewCreateAction(databasemanagersResource, c.ns, databaseManager), &crdtestv1.DatabaseManager{})

	if obj == nil {
		return nil, err
	}
	return obj.(*crdtestv1.DatabaseManager), err
}

// Update takes the representation of a databaseManager and updates it. Returns the server's representation of the databaseManager, and an error, if there is any.
func (c *FakeDatabaseManagers) Update(ctx context.Context, databaseManager *crdtestv1.DatabaseManager, opts v1.UpdateOptions) (result *crdtestv1.DatabaseManager, err error) {
	obj, err := c.Fake.
		Invokes(testing.NewUpdateAction(databasemanagersResource, c.ns, databaseManager), &crdtestv1.DatabaseManager{})

	if obj == nil {
		return nil, err
	}
	return obj.(*crdtestv1.DatabaseManager), err
}

// UpdateStatus was generated because the type contains a Status member.
// Add a +genclient:noStatus comment above the type to avoid generating UpdateStatus().
func (c *FakeDatabaseManagers) UpdateStatus(ctx context.Context, databaseManager *crdtestv1.DatabaseManager, opts v1.UpdateOptions) (*crdtestv1.DatabaseManager, error) {
	obj, err := c.Fake.
		Invokes(testing.NewUpdateSubresourceAction(databasemanagersResource, "status", c.ns, databaseManager), &crdtestv1.DatabaseManager{})

	if obj == nil {
		return nil, err
	}
	return obj.(*crdtestv1.DatabaseManager), err
}

// Delete takes name of the databaseManager and deletes it. Returns an error if one occurs.
func (c *FakeDatabaseManagers) Delete(ctx context.Context, name string, opts v1.DeleteOptions) error {
	_, err := c.Fake.
		Invokes(testing.NewDeleteActionWithOptions(databasemanagersResource, c.ns, name, opts), &crdtestv1.DatabaseManager{})

	return err
}

// DeleteCollection deletes a collection of objects.
func (c *FakeDatabaseManagers) DeleteCollection(ctx context.Context, opts v1.DeleteOptions, listOpts v1.ListOptions) error {
	action := testing.NewDeleteCollectionAction(databasemanagersResource, c.ns, listOpts)

	_, err := c.Fake.Invokes(action, &crdtestv1.DatabaseManagerList{})
	return err
}

// Patch applies the patch and returns the patched databaseManager.
func (c *FakeDatabaseManagers) Patch(ctx context.Context, name string, pt types.PatchType, data []byte, opts v1.PatchOptions, subresources ...string) (result *crdtestv1.DatabaseManager, err error) {
	obj, err := c.Fake.
		Invokes(testing.NewPatchSubresourceAction(databasemanagersResource, c.ns, name, pt, data, subresources...), &crdtestv1.DatabaseManager{})

	if obj == nil {
		return nil, err
	}
	return obj.(*crdtestv1.DatabaseManager), err
}
