// 容器进阶：List / Dictionary / HashSet / Queue / Stack / PriorityQueue / 环形缓冲 / LRU / 去重 / 分组
// 对应 Go golog container_list/ring/heap + slice/map/unique、Rust rustlog array_map.rs

public static class CollectionDemo
{
    public static void Run()
    {
        // ===== 1. 列表 List（对应 Go slice / Rust Vec / Java ArrayList）=====
        Console.WriteLine("--- 列表 List ---");

        var nums = new List<int> { 3, 1, 2 };
        Console.WriteLine($"contains 2? {nums.Contains(2)}");

        nums.Sort();                      // 排序
        Console.WriteLine($"sorted: {string.Join(" ", nums)}");

        nums.Reverse();                   // 反转
        Console.WriteLine($"reversed: {string.Join(" ", nums)}");

        nums.Add(3);
        nums.Add(4);
        Console.WriteLine($"with dup: {string.Join(" ", nums)}");
        Console.WriteLine($"Distinct: {string.Join(" ", nums.Distinct())}"); // 去重（保持顺序）

        nums.RemoveAll(n => n > 2);       // 按条件移除
        Console.WriteLine($"filtered: {string.Join(" ", nums)}");

        // ===== 2. 字典 Dictionary（对应 Go map / Java HashMap）=====
        Console.WriteLine("--- 字典 Dictionary ---");

        var m = new Dictionary<string, int> { ["a"] = 1, ["b"] = 2 };
        Console.WriteLine($"m[b] = {m["b"]}");
        Console.WriteLine($"has c? {m.ContainsKey("c")}");
        m.Remove("a");
        // 按 key 排序输出（对应 Go maps.Sorted）
        Console.WriteLine($"sorted keys: {string.Join(" ", m.Keys.OrderBy(k => k))}");

        // ===== 3. 集合 HashSet（自动去重，对应 Go map[T]struct{} / Python set）=====
        Console.WriteLine("--- 集合 HashSet ---");

        var set = new HashSet<int> { 3, 1, 2, 3, 1 };
        Console.WriteLine($"set: {string.Join(" ", set.OrderBy(x => x))}"); // 1 2 3

        // ===== 4. 双端队列 Queue（对应 Go container/list 双端操作 / C++ deque）=====
        Console.WriteLine("--- 队列 Queue ---");

        var q = new Queue<int>();
        q.Enqueue(1);
        q.Enqueue(2);
        q.Enqueue(3);
        Console.WriteLine($"queue: {string.Join(" ", q)}");
        q.Dequeue(); // 队首出队
        Console.WriteLine($"after dequeue front = {q.Peek()}");

        // ===== 5. 栈 Stack（对应 Go 手写栈 / Java Deque）=====
        Console.WriteLine("--- 栈 Stack ---");

        var st = new Stack<int>();
        st.Push(1);
        st.Push(2);
        st.Push(3);
        Console.WriteLine($"stack pop: {st.Pop()} {st.Pop()} {st.Pop()}"); // 3 2 1（后进先出）

        // ===== 6. 优先队列 PriorityQueue（对应 Go container/heap / C++ priority_queue）=====
        Console.WriteLine("--- 优先队列 PriorityQueue ---");

        // 小顶堆（默认出队最小）；用负数优先级实现大顶堆
        var minHeap = new PriorityQueue<int, int>();
        var maxHeap = new PriorityQueue<int, int>();
        foreach (int n in new[] { 30, 10, 50, 20 })
        {
            minHeap.Enqueue(n, n);
            maxHeap.Enqueue(n, -n); // 负数优先级 → 出队最大
        }
        Console.Write("min-heap pop: ");
        while (minHeap.Count > 0) Console.Write($"{minHeap.Dequeue()} ");
        Console.WriteLine();
        Console.Write("max-heap pop: ");
        while (maxHeap.Count > 0) Console.Write($"{maxHeap.Dequeue()} ");
        Console.WriteLine();

        // ===== 7. 环形缓冲区（手写，对应 Go container/ring）=====
        Console.WriteLine("--- 环形缓冲区 ---");

        var ring = new RingBuffer(5);
        for (int i = 0; i < 5; i++) ring.Write(i);        // 0 1 2 3 4
        Console.WriteLine($"ring: {string.Join(" ", ring.ReadAll())}");
        for (int i = 5; i < 8; i++) ring.Write(i);        // 覆盖 0 1 2 → 3 4 5 6 7
        Console.WriteLine($"after overwrite: {string.Join(" ", ring.ReadAll())}");

        // ===== 8. LRU 缓存（手写，对应 Go lru_test.go）=====
        Console.WriteLine("--- LRU 缓存 ---");

        var lru = new LruCache<string, string>(3);
        lru.Set("key1", "value1");
        lru.Set("key2", "value2");
        lru.Set("key3", "value3");
        Console.WriteLine($"get key2 → {lru.Get("key2")}");
        lru.Set("key4", "value4"); // 淘汰最久未使用的 key1
        Console.WriteLine($"LRU order: {string.Join(" ", lru.KeysInOrder())}"); // key4 key2 key3（最近使用在前）

        // ===== 9. 分组聚合（对应 Go / Python / TS 的 group by）=====
        Console.WriteLine("--- 分组聚合 ---");

        var people = new (string Name, string Dept)[]
        {
            ("Alice", "HR"), ("Bob", "R&D"), ("Charlie", "R&D"), ("David", "HR"),
        };
        foreach (var group in people.GroupBy(p => p.Dept))
        {
            Console.WriteLine($"{group.Key}: {group.Count()} 人");
        }
    }

    // 环形缓冲区：固定容量，写满后覆盖最旧数据（对应 Go container/ring）
    private class RingBuffer
    {
        private readonly int[] _data;
        private int _head; // 写入位置
        private int _count;

        public RingBuffer(int capacity)
        {
            _data = new int[capacity];
        }

        public void Write(int value)
        {
            _data[_head] = value;
            _head = (_head + 1) % _data.Length;
            if (_count < _data.Length) _count++;
        }

        public int[] ReadAll()
        {
            // 从最旧数据开始读：start = (head - count + len) % len，避免负数取模
            int start = (_head - _count + _data.Length) % _data.Length;
            return Enumerable.Range(0, _count).Select(i => _data[(start + i) % _data.Length]).ToArray();
        }
    }

    // LRU 缓存：链表记录访问顺序 + 字典 O(1) 查找（对应 Go lru_test.go）
    private class LruCache<TKey, TValue> where TKey : notnull
    {
        private readonly int _capacity;
        private readonly Dictionary<TKey, LinkedListNode<(TKey Key, TValue Value)>> _map = new();
        private readonly LinkedList<(TKey Key, TValue Value)> _list = new();

        public LruCache(int capacity)
        {
            _capacity = capacity;
        }

        public TValue? Get(TKey key)
        {
            if (!_map.TryGetValue(key, out var node)) return default;
            _list.Remove(node);
            _list.AddFirst(node); // 移到队首 = 最近使用
            return node.Value.Value;
        }

        public void Set(TKey key, TValue value)
        {
            if (_map.TryGetValue(key, out var node))
            {
                node.Value = (key, value);
                _list.Remove(node);
                _list.AddFirst(node);
                return;
            }
            var newNode = _list.AddFirst((key, value));
            _map[key] = newNode;
            if (_map.Count > _capacity)
            {
                _map.Remove(_list.Last!.Value.Key); // 淘汰队尾 = 最久未使用
                _list.RemoveLast();
            }
        }

        public IEnumerable<TKey> KeysInOrder() => _list.Select(n => n.Key);
    }
}
