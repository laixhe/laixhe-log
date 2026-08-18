package com.laixhe.javalog;

import com.laixhe.javalog.demo.CollectionDemo;
import org.junit.jupiter.api.Test;

import java.util.*;

import static org.junit.jupiter.api.Assertions.assertEquals;

/**
 * 集合类型测试（对应 Rust array_map.rs 与 Go container/lru 等测试的练习题）。
 */
class CollectionTest {

    // 练习 1：TreeMap 按 key 有序遍历
    @Test
    void exercise1_tree_map_sorted() {
        Map<String, Integer> map = new TreeMap<>();
        map.put("b", 2);
        map.put("a", 1);
        map.put("c", 3);
        assertEquals(List.of("a", "b", "c"), new ArrayList<>(map.keySet()));
    }

    // 练习 2：PriorityQueue 任务优先级队列（数字越大优先级越高，用最大堆）
    @Test
    void exercise2_priority_queue() {
        PriorityQueue<Integer> queue = new PriorityQueue<>(Comparator.reverseOrder());
        queue.addAll(List.of(1, 2, 3));
        assertEquals(3, queue.poll());
        assertEquals(2, queue.poll());
        assertEquals(1, queue.poll());
    }

    // 练习 3：先排序再相邻去重
    @Test
    void exercise3_sort_then_compact() {
        List<Integer> v = new ArrayList<>(List.of(3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5));
        v.sort(Integer::compareTo);
        List<Integer> deduped = v.stream().distinct().toList();
        assertEquals(List.of(1, 2, 3, 4, 5, 6, 9), deduped);
    }

    // 练习 4：LRU 缓存淘汰最久未使用
    @Test
    void exercise4_lru_cache() {
        CollectionDemo.LRUCache cache = new CollectionDemo.LRUCache(3);
        cache.put("key1", "value1");
        cache.put("key2", "value2");
        cache.put("key3", "value3");
        cache.get("key2"); // 访问 key2，使其变为最近使用

        cache.put("key4", "value4"); // 容量 3，应淘汰 key1
        List<Map.Entry<String, String>> entries = cache.entries();
        assertEquals("key3", entries.get(0).getKey());
        assertEquals("key4", entries.get(entries.size() - 1).getKey());
        assertEquals("", cache.get("key1")); // key1 已被淘汰
        assertEquals("value2", cache.get("key2"));
    }

    // 练习 5：环形缓冲区覆盖最旧数据
    @Test
    void exercise5_ring_buffer() {
        CollectionDemo.RingBuffer ring = new CollectionDemo.RingBuffer(3);
        ring.add(1);
        ring.add(2);
        ring.add(3);
        ring.add(4); // 覆盖最旧的 1
        assertEquals(List.of(2, 3, 4), ring.snapshot());
    }

    // 练习 6：切片排序与结构体排序（对应 Go TestSliceHandle）
    @Test
    void exercise6_slice_sort() {
        List<Integer> order = new ArrayList<>(List.of(2, 1, 3));
        order.sort(Integer::compareTo);
        assertEquals(List.of(1, 2, 3), order);
        order.sort(Comparator.reverseOrder());
        assertEquals(List.of(3, 2, 1), order);

        record Person(String name, int age) {
        }
        List<Person> people = new ArrayList<>(List.of(
                new Person("Jax", 36),
                new Person("TJ", 26),
                new Person("Alex", 76)));
        people.sort(Comparator.comparingInt(Person::age).thenComparing(Person::name));
        assertEquals("TJ", people.get(0).name());
        assertEquals("Alex", people.get(2).name());
    }

    // 练习 7：字典 key 排序（对应 Go TestMapHandle）
    @Test
    void exercise7_map_sorted_keys() {
        Map<String, Integer> m = new HashMap<>();
        m.put("a", 1);
        m.put("b", 2);
        m.put("c", 3);
        List<String> sortedKeys = m.keySet().stream().sorted().toList();
        assertEquals(List.of("a", "b", "c"), sortedKeys);
    }

    // 运行完整 Demo
    @Test
    void runCollectionDemo() {
        CollectionDemo.stdArray();
        CollectionDemo.stdTuple();
        CollectionDemo.stdArrayList();
        CollectionDemo.stdArrayDeque();
        CollectionDemo.stdHashMap();
        CollectionDemo.stdTreeMap();
        CollectionDemo.stdHashSet();
        CollectionDemo.stdTreeSet();
        CollectionDemo.stdPriorityQueue();
        CollectionDemo.stdLinkedList();
        CollectionDemo.ringBuffer();
        CollectionDemo.lruCache();
        CollectionDemo.sliceHandle();
        CollectionDemo.mapHandle();
        CollectionDemo.unique();
        CollectionDemo.grouping();
    }
}
