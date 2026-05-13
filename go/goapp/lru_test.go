package main

import (
	"container/list"
	"fmt"
	"testing"
)

// LRU 缓存实现（核心逻辑）

type LRUData struct {
	key   string
	value string
}

type LRUCache struct {
	capacity int
	cache    map[string]*list.Element
	list     *list.List
}

func NewLRUCache(capacity int) *LRUCache {
	return &LRUCache{
		capacity: capacity,
		cache:    make(map[string]*list.Element),
		list:     list.New(),
	}
}

func (c *LRUCache) Get(key string) string {
	if elem, ok := c.cache[key]; ok {
		c.list.MoveToFront(elem) // 访问后移到队首，提升访问优先级
		return elem.Value.(*LRUData).value
	}
	return ""
}

func (c *LRUCache) Put(key, value string) {
	if elem, ok := c.cache[key]; ok {
		elem.Value.(*LRUData).value = value
		c.list.MoveToFront(elem)
		return
	}

	elem := c.list.PushFront(&LRUData{
		key:   key,
		value: value,
	})
	c.cache[key] = elem

	// 超出容量时删除队尾元素
	if c.list.Len() > c.capacity {
		// 删除尾节点（最久未使用）
		oldElem := c.list.Back()
		c.list.Remove(oldElem)
		delete(c.cache, oldElem.Value.(*LRUData).key)
		oldElem = nil // 手动置空指针，帮助 GC 回收
	}
}

func (c *LRUCache) Show() {
	// 顺序遍历
	for e := c.list.Front(); e != nil; e = e.Next() {
		fmt.Println(e.Value.(*LRUData))
	}
}

func TestLRU(t *testing.T) {
	lRUCache := NewLRUCache(3)
	lRUCache.Put("key1", "value1")
	lRUCache.Put("key2", "value2")
	lRUCache.Put("key3", "value3")

	fmt.Println(lRUCache)
	lRUCache.Show()

	fmt.Println(lRUCache.Get("key2"))

	fmt.Println(lRUCache)
	lRUCache.Show()

	lRUCache.Put("key4", "value4")

	fmt.Println(lRUCache)
	lRUCache.Show()
}
