<?php

namespace Laixhe\Phplog;

use ArrayObject;
use SplDoublyLinkedList;
use SplPriorityQueue;
use SplQueue;
use SplStack;

/**
 * 集合类型示例：数组（索引/关联）、SPL 容器、环形缓冲区、LRU 缓存、去重。
 * 对应 Rust rustlog/src/array_map.rs 与 Go golog 的 slice/map/list/ring/heap/lru/unique。
 *
 * 选型速查：
 * - 默认存一组元素 → 数组（PHP 数组同时是列表 + 字典）
 * - 两端增删 → SplDoublyLinkedList（对应 VecDeque / ArrayDeque）
 * - 无序键值对 → 关联数组；需要有序遍历 → ksort() 后遍历
 * - 每次取最值 → SplPriorityQueue（默认最大堆）
 * - 双向链表 → SplDoublyLinkedList（同 Java LinkedList 场景）
 */
final class CollectionDemo
{
    // ============ 数组 Array / 列表 ============
    public static function stdArray(): void
    {
        // 索引数组（对应 Rust Vec / Java ArrayList）
        $arr = [1, 2, 3, 4, 5];
        echo 'arr = ', implode(',', $arr), PHP_EOL;

        // for 遍历
        echo 'for遍历元素: ';
        foreach ($arr as $element) {
            echo $element, ' ';
        }
        echo PHP_EOL;

        // 索引遍历
        for ($i = 0; $i < count($arr); $i++) {
            echo "索引遍历 {$i}: {$arr[$i]}", PHP_EOL;
        }

        // ⚠️ 越界访问：PHP 不报错，而是返回 null + 警告（区别于 Rust panic）
        $v = $arr[10] ?? '越界返回 null（可用 ?? 兜底）';
        echo "arr[10] = {$v}", PHP_EOL;

        // ---- 常用列表操作 ----
        $v = ['a', 'b', 'c', 'd'];
        array_splice($v, 2, 0, 'x'); // 在索引 2 插入 'x'（对应 insert）
        echo 'after insert: ', implode(',', $v), PHP_EOL; // a,b,x,c,d
        $removed = array_splice($v, 1, 1)[0]; // 删除索引 1 的元素（对应 remove）
        echo "remove index 1 → got '{$removed}', list now: ", implode(',', $v), PHP_EOL; // b, a,x,c,d

        // 尾部增删（O(1) 均摊）
        $stack = [1, 2];
        array_push($stack, 3); // push
        $top = array_pop($stack); // pop
        echo "push/pop: top={$top}, stack=", implode(',', $stack), PHP_EOL; // 3, 1,2

        // 首尾元素（对应 getFirst / getLast）
        $v3 = [1, 2, 3];
        echo 'first=', $v3[0], ' last=', end($v3), PHP_EOL; // 1 3

        // 排序 / 去重（对应 sort + dedup）
        $v4 = [3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5];
        sort($v4); // 升序排序（原地）
        echo 'sorted: ', implode(',', $v4), PHP_EOL;
        $deduped = array_values(array_unique($v4)); // 去重
        echo 'deduped: ', implode(',', $deduped), PHP_EOL; // 1,2,3,4,5,6,9

        // 合并（对应 extend）
        $v5 = [1, 2];
        array_push($v5, ...array_values([3, 4, 5])); // 批量追加
        echo '合并后: ', implode(',', $v5), PHP_EOL; // 1,2,3,4,5

        // 切片（对应 subList / chunks）
        $v6 = [10, 20, 30, 40, 50];
        echo 'array_slice(0,2): ', implode(',', array_slice($v6, 0, 2)), PHP_EOL; // 10,20
        echo 'array_chunk(2): ', json_encode(array_chunk($v6, 2)), PHP_EOL; // [[10,20],[30,40],[50]]

        // 保留偶数（对应 retain）
        $nums = [1, 2, 3, 4, 5, 6];
        $evens = array_values(array_filter($nums, fn ($n) => $n % 2 === 0));
        echo '保留偶数: ', implode(',', $evens), PHP_EOL; // 2,4,6

        // 查找（对应 slices.Contains / indexOf）
        var_dump(in_array(2, [1, 2, 3])); // true
        var_dump(array_search(2, [1, 2, 3])); // 1（索引）
    }

    // ============ 关联数组 ============
    // PHP 数组同时承担 Map 职责，这里展示 map 常用操作
    public static function stdAssocArray(): void
    {
        // ---- HashMap 风格（无序，但 PHP 数组实际按插入序保存）----
        $map = ['Blue' => 10, 'Red' => 25];
        echo '初始: ', json_encode($map, JSON_UNESCAPED_UNICODE), PHP_EOL;

        echo 'get Red=', $map['Red'] ?? '不存在', PHP_EOL; // 25
        var_dump(array_key_exists('Red', $map)); // true
        var_dump(isset($map['Red']));            // true（注意 null 值用 array_key_exists）

        // 删除
        unset($map['Red']);
        echo 'unset Red 后: ', json_encode($map, JSON_UNESCAPED_UNICODE), PHP_EOL; // {"Blue":10}

        // computeIfAbsent（不存在就插入，存在返回旧值）
        $map['Red'] ??= 20; // 对应 entry().or_insert(20)
        echo '??= Red=', $map['Red'], PHP_EOL; // 20

        // 遍历
        echo '遍历: ';
        foreach ($map as $key => $value) {
            echo "{$key} = {$value}; ";
        }
        echo PHP_EOL;

        // ---- TreeMap 风格：ksort 后按 key 有序遍历 ----
        $sorted = ['Charlie' => 30, 'Alice' => 25, 'Bob' => 28, 'David' => 35];
        ksort($sorted);
        echo 'ksort 后（必定有序）: ', implode(',', array_keys($sorted)), PHP_EOL; // Alice,Bob,Charlie,David
    }

    // ============ 双端队列 SplDoublyLinkedList ============
    // 对应 VecDeque / ArrayDeque：两端 O(1) 增删
    public static function stdDeque(): void
    {
        $dq = new SplDoublyLinkedList();
        $dq->push(2);
        $dq->push(3);
        $dq->push(4);
        $dq->unshift(1); // 头部插入（对应 push_front）
        $dq->push(5);    // 尾部插入（对应 push_back）

        echo 'addFirst/addLast 后: ', implode(',', iterator_to_array($dq)), PHP_EOL; // 1,2,3,4,5

        $front = $dq->shift(); // 头部弹出（对应 pop_front）
        $back = $dq->pop();    // 尾部弹出（对应 pop_back）
        echo "shift={$front} pop={$back} 剩余: ", implode(',', iterator_to_array($dq)), PHP_EOL; // 1,5,2,3,4
    }

    // ============ 队列 / 栈 ============
    // 对应 Go container/list 模拟队列 / 栈
    public static function stdQueueStack(): void
    {
        // 队列（FIFO）
        $queue = new SplQueue();
        $queue->enqueue('a');
        $queue->enqueue('b');
        echo '队首元素: ', $queue->dequeue(), PHP_EOL; // a
        echo '队首元素: ', $queue->dequeue(), PHP_EOL; // b

        // 栈（LIFO）
        $stack = new SplStack();
        $stack->push(1);
        $stack->push(2);
        echo '栈顶: ', $stack->pop(), PHP_EOL; // 2
        echo '栈顶: ', $stack->pop(), PHP_EOL; // 1
    }

    // ============ 优先级队列 SplPriorityQueue ============
    // 对应 BinaryHeap：默认最大堆（值越大越先出队）
    public static function stdPriorityQueue(): void
    {
        // ---- 最大堆（默认）----
        $maxHeap = new SplPriorityQueue();
        $maxHeap->insert('任务3', 30);
        $maxHeap->insert('任务1', 10);
        $maxHeap->insert('任务5', 50);
        $maxHeap->insert('任务2', 20);

        echo '最大堆依次 extract: ';
        while (!$maxHeap->isEmpty()) {
            echo $maxHeap->extract(), ' '; // 任务5 任务3 任务2 任务1
        }
        echo PHP_EOL;

        // ---- 最小堆（优先级取负数）----
        $minHeap = new SplPriorityQueue();
        foreach ([30, 10, 50, 20] as $n) {
            $minHeap->insert($n, -$n); // 取负实现最小堆
        }
        echo '最小堆依次 extract: ';
        while (!$minHeap->isEmpty()) {
            echo $minHeap->extract(), ' '; // 10 20 30 50
        }
        echo PHP_EOL;
    }

    // ============ 环形缓冲区 RingBuffer ============
    // 对应 Go container/ring：固定容量环，写满覆盖最旧数据（实现见 src/RingBuffer.php）
    public static function ringBuffer(): void
    {
        $ring = new RingBuffer(5);
        for ($i = 0; $i < 5; $i++) {
            $ring->add($i); // 写入 0-4
        }
        echo '环遍历: ', implode(',', $ring->snapshot()), PHP_EOL; // 0,1,2,3,4

        // 继续写入 3 个，最旧的 0、1、2 被覆盖
        $ring->add(5);
        $ring->add(6);
        $ring->add(7);
        echo '覆盖后环遍历: ', implode(',', $ring->snapshot()), PHP_EOL; // 3,4,5,6,7
    }

    // ============ LRU 缓存（核心逻辑）============
    // 对应 Go lru_test.go：SplDoublyLinkedList（按访问序）+ 关联数组（实现见 src/LRUCache.php）
    public static function lruCache(): void
    {
        $cache = new LRUCache(3);
        $cache->put('key1', 'value1');
        $cache->put('key2', 'value2');
        $cache->put('key3', 'value3');
        $cache->show(); // key1 key2 key3（key1 最久未使用）

        echo 'get key2 → ', $cache->get('key2'), PHP_EOL; // value2
        $cache->show(); // key1 key3 key2（key2 被移到队尾）

        $cache->put('key4', 'value4'); // 容量 3，淘汰 key1
        $cache->show(); // key3 key2 key4
    }

    // ============ 切片处理（对应 Go slice_test.go）============
    public static function sliceHandle(): void
    {
        // 查找（对应 slices.Contains）
        var_dump(in_array(2, [1, 2, 3]));                       // true
        var_dump(in_array('查找', ['切片', '查找', '处理']));     // true

        // 排序（对应 slices.Sort，原地修改）
        $order = [2, 1, 3];
        sort($order); // 升序
        echo implode(',', $order), PHP_EOL; // 1,2,3
        rsort($order); // 从大到小
        echo implode(',', $order), PHP_EOL; // 3,2,1

        // 移除连续重复（对应 slices.Compact）
        $compact = [11, 2, 2, 3, 3, 8, 11];
        $compacted = [];
        foreach ($compact as $n) {
            if (end($compacted) !== $n) { // 只去除相邻重复
                $compacted[] = $n;
            }
        }
        echo implode(',', $compacted), PHP_EOL; // 11,2,3,8,11

        // 结构体排序（对应 slices.SortFunc，年龄升序、相同按姓名升序）
        $people = [
            ['name' => 'Jax', 'age' => 36],
            ['name' => 'TJ', 'age' => 26],
            ['name' => 'Alex', 'age' => 76],
        ];
        usort($people, fn ($a, $b) => $a['age'] <=> $b['age']
            ?: $a['name'] <=> $b['name']); // 先比年龄，再比姓名
        echo json_encode($people, JSON_UNESCAPED_UNICODE), PHP_EOL;
        // [{"name":"TJ","age":26},{"name":"Jax","age":36},{"name":"Alex","age":76}]
    }

    // ============ 字典处理（对应 Go map_test.go）============
    public static function mapHandle(): void
    {
        $m = ['a' => 1, 'b' => 2, 'c' => 3];

        // 获取所有 key
        $keys = array_keys($m);
        echo implode(',', $keys), PHP_EOL; // a,b,c（按插入序）

        // 排序后的 key（对应 slices.Sorted(maps.Keys(m))）
        $sortedKeys = array_keys($m);
        sort($sortedKeys);
        echo implode(',', $sortedKeys), PHP_EOL; // a,b,c
    }

    // ============ 去重（对应 Go unique_test.go）============
    public static function unique(): void
    {
        $nums = [3, 1, 2, 1, 3, 2, 4, 5, 4];

        // array_unique：去重并保持首次出现的顺序
        $seen = array_values(array_unique($nums));
        echo 'array_unique 去重: ', implode(',', $seen), PHP_EOL; // 3,1,2,4,5

        // 手动去重（对应 Go 的 map 记录法）
        $result = [];
        foreach ($nums as $n) {
            $result[$n] = true; // key 天然去重
        }
        echo '手动去重: ', implode(',', array_keys($result)), PHP_EOL; // 3,1,2,4,5
    }

    // ============ 分组聚合（综合实战）============
    public static function grouping(): void
    {
        $staff = [
            ['dept' => 'R&D', 'name' => '张三'],
            ['dept' => 'R&D', 'name' => '李四'],
            ['dept' => 'HR', 'name' => '王五'],
        ];

        // 按部门分组（对应 Java Collectors.groupingBy）
        $byDept = [];
        foreach ($staff as $e) {
            $byDept[$e['dept']][] = $e['name'];
        }
        echo '按部门分组: ', json_encode($byDept, JSON_UNESCAPED_UNICODE), PHP_EOL;
        // {"R&D":["张三","李四"],"HR":["王五"]}

        // 统计各部门人数
        $count = array_map('count', $byDept);
        echo '各部门人数: ', json_encode($count, JSON_UNESCAPED_UNICODE), PHP_EOL; // {"R&D":2,"HR":1}
    }
}
