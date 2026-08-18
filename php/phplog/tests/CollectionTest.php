<?php

namespace Laixhe\Phplog\Tests;

use Laixhe\Phplog\CollectionDemo;
use Laixhe\Phplog\LRUCache;
use Laixhe\Phplog\RingBuffer;
use PHPUnit\Framework\TestCase;

/**
 * 集合类型测试（对应 Rust array_map.rs 与 Go container/lru 等测试的练习题）。
 */
final class CollectionTest extends TestCase
{
    // 练习 1：关联数组按 key 有序遍历（对应 BTreeMap 练习）
    public function testExercise1SortedKeys(): void
    {
        $map = ['b' => 2, 'a' => 1, 'c' => 3];
        ksort($map);
        $this->assertSame(['a', 'b', 'c'], array_keys($map));
    }

    // 练习 2：SplPriorityQueue 优先级队列（数字越大优先级越高）
    public function testExercise2PriorityQueue(): void
    {
        $queue = new \SplPriorityQueue();
        $queue->insert('低', 1);
        $queue->insert('高', 3);
        $queue->insert('中', 2);
        $this->assertSame('高', $queue->extract());
        $this->assertSame('中', $queue->extract());
        $this->assertSame('低', $queue->extract());
    }

    // 练习 3：先排序再相邻去重（对应 sort + dedup）
    public function testExercise3SortThenCompact(): void
    {
        $v = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        sort($v);
        $deduped = array_values(array_unique($v));
        $this->assertSame([1, 2, 3, 4, 5, 6, 9], $deduped);
    }

    // 练习 4：LRU 缓存淘汰最久未使用
    public function testExercise4LruCache(): void
    {
        $cache = new LRUCache(3);
        $cache->put('key1', 'value1');
        $cache->put('key2', 'value2');
        $cache->put('key3', 'value3');
        $cache->get('key2'); // 访问 key2，使其变为最近使用

        $cache->put('key4', 'value4'); // 容量 3，应淘汰 key1
        $entries = array_keys($cache->entries());
        $this->assertNotContains('key1', $entries);
        $this->assertSame(['key3', 'key2', 'key4'], $entries);
        $this->assertSame('', $cache->get('key1')); // key1 已被淘汰
        $this->assertSame('value2', $cache->get('key2'));
    }

    // 练习 5：环形缓冲区覆盖最旧数据
    public function testExercise5RingBuffer(): void
    {
        $ring = new RingBuffer(3);
        $ring->add(1);
        $ring->add(2);
        $ring->add(3);
        $ring->add(4); // 覆盖最旧的 1
        $this->assertSame([2, 3, 4], $ring->snapshot());
    }

    // 练习 6：切片排序与结构体排序（对应 Go TestSliceHandle）
    public function testExercise6SliceSort(): void
    {
        $order = [2, 1, 3];
        sort($order);
        $this->assertSame([1, 2, 3], $order);
        rsort($order);
        $this->assertSame([3, 2, 1], $order);

        $people = [
            ['name' => 'Jax', 'age' => 36],
            ['name' => 'TJ', 'age' => 26],
            ['name' => 'Alex', 'age' => 76],
        ];
        usort($people, fn ($a, $b) => $a['age'] <=> $b['age']
            ?: $a['name'] <=> $b['name']);
        $this->assertSame('TJ', $people[0]['name']);
        $this->assertSame('Alex', $people[2]['name']);
    }

    // 运行完整 Demo
    public function testRunCollectionDemo(): void
    {
        $this->expectNotToPerformAssertions();
        CollectionDemo::stdArray();
        CollectionDemo::stdAssocArray();
        CollectionDemo::stdDeque();
        CollectionDemo::stdQueueStack();
        CollectionDemo::stdPriorityQueue();
        CollectionDemo::ringBuffer();
        CollectionDemo::lruCache();
        CollectionDemo::sliceHandle();
        CollectionDemo::mapHandle();
        CollectionDemo::unique();
        CollectionDemo::grouping();
    }
}
