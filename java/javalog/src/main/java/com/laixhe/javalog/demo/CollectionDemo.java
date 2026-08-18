package com.laixhe.javalog.demo;

import java.util.*;
import java.util.function.Function;
import java.util.stream.Collectors;

/**
 * 集合类型示例：数组、记录（元组）、ArrayList、ArrayDeque、HashMap、TreeMap、
 * HashSet、TreeSet、PriorityQueue、LinkedList、环形缓冲区、LRU 缓存、去重。
 * 对应 Rust rustlog/src/array_map.rs 与 Go golog 的 slice/map/list/ring/heap/lru/unique。
 *
 * 选型速查：
 * - 默认存一组元素 → ArrayList
 * - 两端增删 → ArrayDeque（对应 VecDeque，内部环形缓冲）
 * - 无序键值对 → HashMap；需要有序遍历/范围查询 → TreeMap
 * - 无序不重复值 → HashSet；有序不重复值 → TreeSet
 * - 每次取最值 → PriorityQueue（默认最小堆）
 * - 双向链表 → LinkedList（⚠️ 绝大多数场景应优先用 ArrayList / ArrayDeque）
 */
public final class CollectionDemo {

    private CollectionDemo() {
    }

    // ============ 数组 Array ============
    public static void stdArray() {
        // 声明一个包含 5 个 int 类型元素的数组（长度固定）
        int[] arr = {1, 2, 3, 4, 5};
        System.out.println("arr = " + Arrays.toString(arr));

        // for-each 遍历
        System.out.print("for遍历元素：");
        for (int element : arr) {
            System.out.print(element + " ");
        }
        System.out.println();

        // 声明一个包含 10 个元素、每个元素值为 0 的数组
        int[] arr1 = new int[10]; // 默认填充 0
        System.out.println("arr1 = " + Arrays.toString(arr1));

        // 索引遍历
        for (int i = 0; i < arr1.length; i++) {
            System.out.println("索引遍历 " + i + ": " + arr1[i]);
        }

        // ⚠️ 下标越界会抛 ArrayIndexOutOfBoundsException（对应 Rust 直接索引 panic）
        try {
            @SuppressWarnings("unused")
            int v = arr[10];
        } catch (ArrayIndexOutOfBoundsException e) {
            System.out.println("arr = 索引越界了（ArrayIndexOutOfBoundsException）");
        }
    }

    // ============ 元组 Tuple ============
    // Java 没有元组，用 record（不可变）组合不同类型，效果等价
    public record Tuple(int a, double b, String c) {
    }

    public static void stdTuple() {
        // 定义一个包含整数、浮点数和字符串的元组
        Tuple tup = new Tuple(10, 3.14, "hello");
        System.out.println("tup 元素：" + tup.a() + "、" + tup.b() + "、" + tup.c());

        // 另一个元组，解构（通过访问器方法）
        Tuple another = new Tuple(42, 0, "x");
        int a = another.a();
        double b = another.b();
        String c = another.c();
        System.out.println("another_tup 元素：a = " + a + ", b = " + b + ", c = " + c);
    }

    // ============ 动态数组 ArrayList<T> ============
    // 尾部操作 O(1) 均摊；中间插入/删除 O(n)
    public static void stdArrayList() {
        // ---- 创建方式 ----
        List<Integer> numbers1 = new ArrayList<>();
        System.out.println("numbers1 = " + numbers1); // []

        // 预分配容量（避免频繁扩容，性能优化建议）
        List<Integer> numbers2 = new ArrayList<>(10);
        System.out.println("numbers2 = " + numbers2 + " 大小：" + numbers2.size()); // [] 大小：0

        List<Integer> numbers3 = new ArrayList<>(List.of(1, 2, 3, 4, 5));
        System.out.println("numbers3 = " + numbers3); // [1, 2, 3, 4, 5]

        // ---- 基础查询 ----
        System.out.println("contains 3? " + numbers3.contains(3) + "  contains 6? " + numbers3.contains(6)); // true false

        // ---- 常用方法 ----
        // insert / remove：任意位置插入 / 删除（O(n)）
        List<Character> v = new ArrayList<>(List.of('a', 'b', 'c', 'd'));
        v.add(2, 'x'); // 在索引 2 插入 'x'，后面的元素后移
        System.out.println("after insert: " + v); // [a, b, x, c, d]
        Character removed = v.remove(1); // 删除索引 1 的元素 'b'
        System.out.println("remove index 1 → got '" + removed + "', list now: " + v); // b, [a, x, c, d]

        // swapRemove：快速删除（把最后一个元素移到删除位置，O(1)，但会打乱顺序）
        List<Integer> v2 = new ArrayList<>(List.of(10, 20, 30, 40, 50));
        Integer got = v2.set(1, v2.remove(v2.size() - 1)); // 等价 swap_remove(1)：删掉 20，把 50 搬过来
        System.out.println("swapRemove index 1 → got " + got + ", list: " + v2); // 20, [10, 50, 30, 40]

        // getFirst / getLast（Java 21+）：安全获取首尾元素
        List<Integer> v3 = new ArrayList<>(List.of(1, 2, 3));
        System.out.println("first=" + v3.getFirst() + " last=" + v3.getLast()); // 1 3

        // sort / distinct（去重）：对应 sort + dedup
        List<Integer> v4 = new ArrayList<>(List.of(3, 1, 4, 1, 5, 9, 2, 6, 5, 3, 5));
        v4.sort(Integer::compareTo); // 升序排序（对应 v4.sort()）
        System.out.println("sorted: " + v4);
        List<Integer> deduped = v4.stream().distinct().toList(); // 去重（对应 dedup，但保留全部唯一值）
        System.out.println("deduped: " + deduped); // [1,2,3,4,5,6,9]

        // addAll：批量追加（对应 extend）
        List<Integer> v5 = new ArrayList<>(List.of(1, 2));
        v5.addAll(List.of(3, 4, 5));
        System.out.println("addAll: " + v5); // [1,2,3,4,5]

        // subList：切片（对应 chunks/windows 的基础）
        List<Integer> v6 = new ArrayList<>(List.of(10, 20, 30, 40, 50));
        System.out.println("subList(0,2): " + v6.subList(0, 2)); // [10, 20]

        // retainAll：保留符合条件的元素（对应 retain）
        List<Integer> nums = new ArrayList<>(List.of(1, 2, 3, 4, 5, 6));
        nums.removeIf(n -> n % 2 != 0); // 移除奇数，等价「保留偶数」
        System.out.println("保留偶数：" + nums); // [2, 4, 6]

        // 排序与查找
        List<Integer> sorted = new ArrayList<>(List.of(3, 1, 2));
        sorted.sort(Comparator.reverseOrder()); // 倒序
        System.out.println("倒序：" + sorted); // [3, 2, 1]
        System.out.println("indexOf(2)： " + sorted.indexOf(2)); // 1
    }

    // ============ 双端队列 ArrayDeque<T> ============
    // 对应 VecDeque：头部/尾部 O(1) 增删，内部环形缓冲区
    public static void stdArrayDeque() {
        ArrayDeque<Integer> dq = new ArrayDeque<>(List.of(2, 3, 4));
        dq.addFirst(1);  // 头部插入 O(1)（对应 push_front）
        dq.addLast(5);   // 尾部插入 O(1)（对应 push_back）
        System.out.println("addFirst/addLast 后: " + dq); // [1, 2, 3, 4, 5]

        Integer front = dq.pollFirst(); // 头部弹出 O(1)（对应 pop_front）
        Integer back = dq.pollLast();   // 尾部弹出 O(1)（对应 pop_back）
        System.out.println("pollFirst=" + front + " pollLast=" + back + " 剩余: " + dq); // 1, 5, [2, 3, 4]
    }

    // ============ HashMap<K, V>（无序，哈希表实现）============
    // 对应 HashMap：平均 O(1) 插入/查找/删除，遍历顺序不保证
    public static void stdHashMap() {
        Map<String, Integer> map = new HashMap<>();
        map.put("Blue", 10);
        map.put("Red", 25);
        System.out.println("初始: " + map); // {Blue=10, Red=25}

        // 多种查询方式
        System.out.println("get Red=" + map.get("Red")); // 25
        System.out.println("getOrDefault: " + map.getOrDefault("Green", -1)); // -1
        System.out.println("Red 是否存在: " + map.containsKey("Red")); // true

        // 删除数据
        map.remove("Red");
        System.out.println("remove Red 后: " + map); // {Blue=10}

        // computeIfAbsent：不存在就插入，存在则返回旧值（对应 entry().or_insert）
        int orDefault = map.computeIfAbsent("Red", k -> 20);
        System.out.println("computeIfAbsent: Red=" + orDefault); // 20

        // 遍历（默认不保证顺序）
        System.out.print("遍历 HashMap（顺序可能变）: ");
        map.forEach((key, value) -> System.out.print(key + " = " + value + "; "));
        System.out.println();
    }

    // ============ TreeMap<K, V>（有序，红黑树实现）============
    // 对应 BTreeMap：按 key 自然顺序遍历，支持范围查询
    public static void stdTreeMap() {
        Map<String, Integer> map = new TreeMap<>();
        map.put("Charlie", 30);
        map.put("Alice", 25);
        map.put("Bob", 28);
        map.put("David", 35);

        // 遍历是有序的（按 key 字典序）
        System.out.println("TreeMap 顺序遍历（必定有序）:");
        map.forEach((name, age) -> System.out.println("  " + name + ": " + age)); // Alice→Bob→Charlie→David

        // 范围查询（HashMap 做不到）：查 "B" 到 "D" 之间（半开区间 [B, D)）
        System.out.println("TreeMap 范围查询 B..D:");
        ((TreeMap<String, Integer>) map).subMap("B", "D")
                .forEach((name, age) -> System.out.println("  " + name + ": " + age)); // Bob, Charlie
    }

    // ============ HashSet<T>（无序不重复集合）============
    // 对应 HashSet：平均 O(1) 插入/查找/删除，支持集合运算
    public static void stdHashSet() {
        Set<Integer> a = new HashSet<>(List.of(1, 2, 3, 4));
        Set<Integer> b = new HashSet<>(List.of(3, 4, 5, 6));

        // 插入重复元素会被忽略（返回 false 表示已存在）
        boolean inserted = a.add(2); // 2 已存在
        System.out.println("插入重复 2 成功？" + inserted + "  集合 a=" + a); // false

        // 集合运算
        Set<Integer> inter = new HashSet<>(a);
        inter.retainAll(b); // 交集 a∩b
        System.out.println("交集 a∩b: " + inter); // [3,4]

        Set<Integer> union = new HashSet<>(a);
        union.addAll(b); // 并集 a∪b
        System.out.println("并集 a∪b: " + union); // [1,2,3,4,5,6]

        Set<Integer> diff = new HashSet<>(a);
        diff.removeAll(b); // 差集 a-b
        System.out.println("差集 a-b: " + diff); // [1,2]

        Set<Integer> sym = new HashSet<>(a);
        sym.addAll(b);
        Set<Integer> interCopy = new HashSet<>(a);
        interCopy.retainAll(b);
        sym.removeAll(interCopy); // 对称差 (a-b)∪(b-a)
        System.out.println("对称差 (a-b)∪(b-a): " + sym); // [1,2,5,6]
    }

    // ============ TreeSet<T>（有序不重复集合）============
    // 对应 BTreeSet：遍历按自然顺序，支持范围查询
    public static void stdTreeSet() {
        Set<Integer> set = new TreeSet<>(List.of(50, 10, 30, 40, 20));
        System.out.println("TreeSet 有序遍历: " + set); // [10, 20, 30, 40, 50]

        // 范围查询：取 [20, 40) 区间内的值
        System.out.println("TreeSet range 20..40: " + ((TreeSet<Integer>) set).subSet(20, 40)); // [20, 30]
    }

    // ============ PriorityQueue<T>（优先级队列 / 最小堆）============
    // 对应 BinaryHeap：堆顶始终是最值。默认最小堆，Comparator.reverseOrder() 变最大堆
    public static void stdPriorityQueue() {
        // ---- 最小堆（默认，对应 Go container/heap 默认最小堆）----
        PriorityQueue<Integer> minHeap = new PriorityQueue<>();
        minHeap.addAll(List.of(30, 10, 50, 20));
        System.out.print("最小堆依次 poll: ");
        while (!minHeap.isEmpty()) {
            System.out.print(minHeap.poll() + " "); // 10 20 30 50
        }
        System.out.println();

        // ---- 最大堆（Comparator.reverseOrder()）----
        PriorityQueue<Integer> maxHeap = new PriorityQueue<>(Comparator.reverseOrder());
        maxHeap.addAll(List.of(30, 10, 50, 20));
        System.out.print("最大堆依次 poll: ");
        while (!maxHeap.isEmpty()) {
            System.out.print(maxHeap.poll() + " "); // 50 30 20 10
        }
        System.out.println();

        // ---- peek：查看堆顶（不弹出）----
        PriorityQueue<Integer> h = new PriorityQueue<>(List.of(3, 1, 4, 1, 5));
        System.out.println("peek 最小值 = " + h.peek()); // 1
    }

    // ============ 双向链表 LinkedList<T> ============
    // 对应 Go container/list：头部/尾部/指定节点前后插入删除
    public static void stdLinkedList() {
        LinkedList<String> l = new LinkedList<>();
        l.addLast("first");  // 尾部插入（PushBack）
        l.addFirst("last");  // 头部插入（PushFront）
        int index = l.indexOf("first");
        l.add(index, "middle"); // 在 "first" 前插入（InsertBefore）

        System.out.println("链表长度: " + l.size());            // 3
        System.out.println("头元素值: " + l.getFirst());        // last
        System.out.println("尾元素值: " + l.getLast());         // first

        System.out.println("=============================");

        // 用 LinkedList 模拟队列（对应 Go queue := list.New()）
        LinkedList<String> queue = new LinkedList<>();
        queue.addLast("a"); // 队尾：a
        queue.addLast("b"); // 队尾：b
        System.out.println("队首元素：" + queue.getFirst()); // a

        while (!queue.isEmpty()) {
            System.out.println(queue.removeFirst()); // 弹出 a, b
        }
    }

    // ============ 环形缓冲区 RingBuffer ============
    // 对应 Go container/ring：固定容量环，写满覆盖最旧数据
    public static final class RingBuffer {
        private final Object[] buf;
        private final int capacity;
        private int head; // 下一个写入位置
        private int size;

        public RingBuffer(int capacity) {
            this.capacity = capacity;
            this.buf = new Object[capacity];
        }

        // 写入：环满则覆盖最旧数据
        public void add(Object value) {
            buf[head] = value;
            head = (head + 1) % capacity;
            if (size < capacity) {
                size++;
            }
        }

        // 从最旧到最新遍历
        public List<Object> snapshot() {
            List<Object> list = new ArrayList<>(size);
            int start = (head - size + capacity) % capacity;
            for (int i = 0; i < size; i++) {
                list.add(buf[(start + i) % capacity]);
            }
            return list;
        }

        public int size() {
            return size;
        }
    }

    public static void ringBuffer() {
        // 对应 Go container/ring：创建包含 5 个节点的环并赋值 0-4
        RingBuffer ring = new RingBuffer(5);
        for (int i = 0; i < 5; i++) {
            ring.add(i); // 写入 0-4
        }
        System.out.println("环遍历: " + ring.snapshot()); // [0, 1, 2, 3, 4]

        // 继续写入 3 个，最旧的 0、1、2 被覆盖
        ring.add(5);
        ring.add(6);
        ring.add(7);
        System.out.println("覆盖后环遍历: " + ring.snapshot()); // [3, 4, 5, 6, 7]
    }

    // ============ LRU 缓存（核心逻辑）============
    // 对应 Go lru_test.go：LinkedHashMap 的 accessOrder=true 天然实现 LRU
    public static final class LRUCache {
        private final int capacity;
        // accessOrder=true：按访问顺序排序，最久未访问的在最前（会被淘汰）
        private final LinkedHashMap<String, String> cache;

        public LRUCache(int capacity) {
            this.capacity = capacity;
            this.cache = new LinkedHashMap<>(capacity, 0.75f, true) {
                // 插入后判断是否超过容量，超过则淘汰最久未访问（队首）
                @Override
                protected boolean removeEldestEntry(Map.Entry<String, String> eldest) {
                    return size() > LRUCache.this.capacity;
                }
            };
        }

        public String get(String key) {
            return cache.getOrDefault(key, ""); // 访问会移动到队尾，提升优先级
        }

        public void put(String key, String value) {
            cache.put(key, value);
        }

        public List<Map.Entry<String, String>> entries() {
            return new ArrayList<>(cache.entrySet());
        }

        public void show() {
            // 顺序遍历：队首是最久未使用
            cache.forEach((k, v) -> System.out.println(k + "=" + v));
        }
    }

    public static void lruCache() {
        LRUCache lRUCache = new LRUCache(3);
        lRUCache.put("key1", "value1");
        lRUCache.put("key2", "value2");
        lRUCache.put("key3", "value3");
        lRUCache.show(); // key1 key2 key3（key1 最久未使用）

        System.out.println("get key2 → " + lRUCache.get("key2")); // value2
        lRUCache.show(); // key1 key3 key2（key2 被移到队尾）

        lRUCache.put("key4", "value4"); // 容量 3，淘汰 key1
        lRUCache.show(); // key3 key2 key4
    }

    // ============ 切片处理（对应 Go slice_test.go）============
    public static void sliceHandle() {
        // 查找（对应 slices.Contains）
        System.out.println(List.of(1, 2, 3).contains(2));                 // true
        System.out.println(List.of("切片", "查找", "处理").contains("查找")); // true

        // 排序（对应 slices.Sort，原地修改）
        List<Integer> order = new ArrayList<>(List.of(2, 1, 3));
        order.sort(Integer::compareTo); // 升序
        System.out.println(order);      // [1, 2, 3]
        order.sort(Comparator.reverseOrder()); // 从大到小
        System.out.println(order);      // [3, 2, 1]

        // 移除连续重复（对应 slices.Compact）
        List<Integer> compact = new ArrayList<>(List.of(11, 2, 2, 3, 3, 8, 11));
        List<Integer> compacted = new ArrayList<>();
        for (Integer n : compact) {
            if (compacted.isEmpty() || !compacted.getLast().equals(n)) {
                compacted.add(n); // 只去除相邻重复
            }
        }
        System.out.println(compacted); // [11, 2, 3, 8, 11]

        // 结构体排序（对应 slices.SortFunc，年龄升序、相同按姓名升序）
        record Person(String name, int age) {
        }
        List<Person> people = new ArrayList<>(List.of(
                new Person("Jax", 36),
                new Person("TJ", 26),
                new Person("Alex", 76)));
        people.sort(Comparator.comparingInt(Person::age).thenComparing(Person::name));
        System.out.println(people); // [Person[name=TJ, age=26], Person[name=Jax, age=36], Person[name=Alex, age=76]]
    }

    // ============ 字典处理（对应 Go map_test.go）============
    public static void mapHandle() {
        Map<String, Integer> m = new HashMap<>();
        m.put("a", 1);
        m.put("b", 2);
        m.put("c", 3);

        // 获取所有 key（对应 maps.Keys → 切片）
        List<String> keys = new ArrayList<>(m.keySet());
        System.out.println(keys); // [a, c, b]（顺序不保证）

        // 排序后的 key（对应 slices.Sorted(maps.Keys(m))）
        List<String> sortedKeys = keys.stream().sorted().toList();
        System.out.println(sortedKeys); // [a, b, c]
    }

    // ============ 去重（对应 Go unique_test.go）============
    public static void unique() {
        int[] nums = {3, 1, 2, 1, 3, 2, 4, 5, 4};

        // LinkedHashSet：去重且保持插入顺序
        Set<Integer> seen = new LinkedHashSet<>();
        for (int n : nums) {
            seen.add(n);
        }
        System.out.println("LinkedHashSet 去重: " + seen); // [3, 1, 2, 4, 5]（保持顺序）

        // 使用流一行去重
        List<Integer> distinct = Arrays.stream(nums).boxed().distinct().toList();
        System.out.println("Stream distinct: " + distinct); // [3, 1, 2, 4, 5]
    }

    // ============ 分组聚合（综合实战）============
    // 对应 Rust practice 前的 groupingBy 示例
    public static void grouping() {
        record Employee(String dept, String name) {
        }
        List<Employee> staff = List.of(
                new Employee("R&D", "张三"),
                new Employee("R&D", "李四"),
                new Employee("HR", "王五"));

        // Collectors.groupingBy：按部门分组（对应 Rust 迭代器综合应用）
        Map<String, List<Employee>> byDept = staff.stream()
                .collect(Collectors.groupingBy(Employee::dept));
        System.out.println("按部门分组: " + byDept);

        // toMap：按部门统计人数
        Map<String, Long> count = staff.stream()
                .collect(Collectors.groupingBy(Employee::dept, Collectors.counting()));
        System.out.println("各部门人数: " + count); // {R&D=2, HR=1}
    }
}
