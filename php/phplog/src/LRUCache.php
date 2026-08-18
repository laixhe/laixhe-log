<?php

namespace Laixhe\Phplog;

use SplDoublyLinkedList;

/**
 * LRU 缓存（对应 Go lru_test.go）：SplDoublyLinkedList（按访问序）+ 关联数组。
 */
final class LRUCache
{
    private int $capacity;
    /** @var array<string, mixed> */
    private array $cache = [];
    private SplDoublyLinkedList $order; // 队尾 = 最近使用，队首 = 最久未使用

    public function __construct(int $capacity)
    {
        $this->capacity = $capacity;
        $this->order = new SplDoublyLinkedList();
    }

    public function get(string $key): mixed
    {
        if (!isset($this->cache[$key])) {
            return '';
        }
        $this->moveToBack($key); // 访问后移到队尾，提升优先级
        return $this->cache[$key];
    }

    public function put(string $key, mixed $value): void
    {
        if (isset($this->cache[$key])) {
            $this->cache[$key] = $value;
            $this->moveToBack($key);
            return;
        }
        $this->cache[$key] = $value;
        $this->order->push($key);

        // 超出容量时删除队首元素（最久未使用）
        if ($this->order->count() > $this->capacity) {
            $oldest = $this->order->shift();
            unset($this->cache[$oldest]);
        }
    }

    private function moveToBack(string $key): void
    {
        // 从链表中移除 key（SplDoublyLinkedList 没有按值删除，用迭代重建）
        $newOrder = new SplDoublyLinkedList();
        foreach ($this->order as $k) {
            if ($k !== $key) {
                $newOrder->push($k);
            }
        }
        $newOrder->push($key);
        $this->order = $newOrder;
    }

    /** 从最久未使用到最近使用遍历 */
    public function entries(): array
    {
        $entries = [];
        foreach ($this->order as $k) {
            $entries[$k] = $this->cache[$k];
        }
        return $entries;
    }

    public function show(): void
    {
        foreach ($this->entries() as $k => $v) {
            echo "{$k}={$v}", PHP_EOL;
        }
    }
}
