#include "StdContainer.h"

#include <algorithm>   // std::ranges::sort / std::unique
#include <deque>       // std::deque
#include <format>      // std::format [C++20]
#include <iostream>
#include <list>        // std::list
#include <map>         // std::map
#include <queue>       // std::priority_queue
#include <ranges>      // std::ranges::contains [C++23]
#include <set>         // std::set
#include <string>
#include <unordered_map>
#include <vector>

// 注：MinGW 的 libstdc++ 暂缺 <print> 的终端符号，这里统一用 std::format + std::cout 输出。
#define PRINT(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__) << std::endl
#define PRINTF(fmt, ...) std::cout << std::format(fmt, ##__VA_ARGS__)

namespace
{
    // 环形缓冲区（对应 Go container/ring）：固定容量环，写满覆盖最旧数据
    class RingBuffer
    {
        public:
        explicit RingBuffer(std::size_t capacity)
            : capacity_(capacity), buf_(capacity) {}

        void Add(int value)
        {
            buf_[head_] = value;
            head_ = (head_ + 1) % capacity_;
            if (size_ < capacity_) size_++;
        }

        std::vector<int> Snapshot() const
        {
            std::vector<int> out;
            const std::size_t start = (head_ + capacity_ - size_) % capacity_;
            for (std::size_t i = 0; i < size_; i++) {
                out.push_back(buf_[(start + i) % capacity_]);
            }
            return out;
        }

        private:
        std::size_t capacity_;
        std::size_t head_ = 0;
        std::size_t size_ = 0;
        std::vector<int> buf_;
    };

    // LRU 缓存（对应 Go lru_test.go）：std::list 记录访问序 + std::unordered_map O(1) 查找
    class LRUCache
    {
        public:
        explicit LRUCache(std::size_t capacity) : capacity_(capacity) {}

        std::string Get(const std::string& key)
        {
            auto it = cache_.find(key);
            if (it == cache_.end()) return "";
            // 访问后移到队尾，提升优先级（对应 MoveToBack）
            order_.splice(order_.end(), order_, it->second.second);
            return it->second.first;
        }

        void Put(const std::string& key, const std::string& value)
        {
            auto it = cache_.find(key);
            if (it != cache_.end()) {
                it->second.first = value;
                order_.splice(order_.end(), order_, it->second.second);
                return;
            }
            order_.push_back(key);
            auto list_it = std::prev(order_.end());
            cache_[key] = {value, list_it};

            // 超出容量时淘汰队首（最久未使用）
            if (cache_.size() > capacity_) {
                const std::string oldest = order_.front();
                order_.pop_front();
                cache_.erase(oldest);
            }
        }

        std::vector<std::string> Keys() const { return {order_.begin(), order_.end()}; }

        private:
        std::size_t capacity_;
        std::list<std::string> order_; // 队首 = 最久未使用，队尾 = 最近使用
        std::unordered_map<std::string, std::pair<std::string, std::list<std::string>::iterator>> cache_;
    };
} // namespace

StdContainer::StdContainer()
{
    // ===== 1. 数组 vector（对应 Go slice_test.go / Rust Vec）=====
    std::cout << "--- 数组 vector ---" << std::endl;

    std::vector<int> v{2, 1, 3};
    // 查找（对应 slices.Contains，std::ranges::contains [C++23]）
    PRINT("contains 2? {}", std::ranges::contains(v, 2));

    // 排序（对应 slices.Sort，原地升序）
    std::ranges::sort(v);
    for (const int n : v) PRINTF("{} ", n);
    std::cout << " <- sorted" << std::endl;
    // 反转（对应 slices.Reverse）
    std::ranges::reverse(v);
    for (const int n : v) PRINTF("{} ", n);
    std::cout << " <- reversed" << std::endl;

    // 去重（对应 unique_test.go：先排序再 unique + erase）
    std::vector<int> nums{3, 1, 2, 1, 3, 2, 4, 5, 4};
    std::ranges::sort(nums);
    auto [first, last] = std::ranges::unique(nums);
    nums.erase(first, last);
    for (const int n : nums) PRINTF("{} ", n);
    std::cout << " <- unique（已排序）" << std::endl;

    // 移除连续重复（对应 slices.Compact，只去除相邻重复）
    std::vector<int> compact{11, 2, 2, 3, 3, 8, 11};
    std::vector<int> compacted;
    for (std::size_t i = 0; i < compact.size(); i++) {
        if (i == 0 || compact[i] != compact[i - 1]) compacted.push_back(compact[i]);
    }
    for (const int n : compacted) PRINTF("{} ", n);
    std::cout << " <- compact（相邻去重）" << std::endl;

    // ===== 2. 字典 map（对应 Go map_test.go / Rust HashMap）=====
    std::cout << "--- 字典 map ---" << std::endl;
    std::map<std::string, int> m{{"a", 1}, {"b", 2}, {"c", 3}};
    PRINT("m[b] = {}", m["b"]);
    m.erase("c");
    PRINT("has c? {}", m.contains("c")); // C++20

    // 排序后的 key（对应 slices.Sorted(maps.Keys(m))，map 天然有序）
    for (const auto& [k, v] : m) PRINTF("{}={} ", k, v);
    std::cout << " <- sorted keys" << std::endl;

    // ===== 3. 集合 set（对应 unique_test.go / Rust HashSet）=====
    std::cout << "--- 集合 set ---" << std::endl;
    std::set<int> set{3, 1, 2, 1, 3};
    for (const int n : set) PRINTF("{} ", n);
    std::cout << " <- set 自动去重且有序" << std::endl;

    // ===== 4. 双端队列 deque（对应 Go container/list 作队列 / Rust VecDeque）=====
    std::cout << "--- 双端队列 deque ---" << std::endl;
    std::deque<int> dq{2, 3, 4};
    dq.push_front(1); // 头部插入（push_front）
    dq.push_back(5);  // 尾部插入（push_back）
    for (const int n : dq) PRINTF("{} ", n);
    std::cout << " <- deque" << std::endl;
    dq.pop_front();
    PRINT("pop_front 后队首 = {}", dq.front());

    // ===== 5. 双向链表 list（对应 Go container/list）=====
    std::cout << "--- 双向链表 list ---" << std::endl;
    std::list<std::string> queue{"a", "b"};
    PRINT("队首元素: {}", queue.front());
    queue.pop_front();
    PRINT("弹出后队首: {}", queue.front());

    // ===== 6. 堆 priority_queue（对应 Go container/heap，默认最大堆）=====
    std::cout << "--- 堆 priority_queue ---" << std::endl;
    std::priority_queue<int> max_heap; // 默认最大堆
    for (const int n : {30, 10, 50, 20}) max_heap.push(n);
    std::cout << "最大堆依次 pop: ";
    while (!max_heap.empty()) {
        PRINTF("{} ", max_heap.top());
        max_heap.pop();
    }
    std::cout << std::endl;

    // 最小堆：greater 比较器
    std::priority_queue<int, std::vector<int>, std::greater<int>> min_heap;
    for (const int n : {30, 10, 50, 20}) min_heap.push(n);
    std::cout << "最小堆依次 pop: ";
    while (!min_heap.empty()) {
        PRINTF("{} ", min_heap.top());
        min_heap.pop();
    }
    std::cout << std::endl;

    // ===== 7. 环形缓冲区（对应 Go container/ring）=====
    std::cout << "--- 环形缓冲区 ---" << std::endl;
    RingBuffer ring(5);
    for (int i = 0; i < 5; i++) ring.Add(i);
    for (const int n : ring.Snapshot()) PRINTF("{} ", n);
    std::cout << " <- 环遍历" << std::endl;
    ring.Add(5);
    ring.Add(6);
    ring.Add(7); // 覆盖最旧的 0、1、2
    for (const int n : ring.Snapshot()) PRINTF("{} ", n);
    std::cout << " <- 覆盖后" << std::endl;

    // ===== 8. LRU 缓存（对应 Go lru_test.go）=====
    std::cout << "--- LRU 缓存 ---" << std::endl;
    LRUCache cache(3);
    cache.Put("key1", "value1");
    cache.Put("key2", "value2");
    cache.Put("key3", "value3");
    PRINT("get key2 → {}", cache.Get("key2")); // value2
    cache.Put("key4", "value4"); // 容量 3，淘汰 key1
    for (const auto& k : cache.Keys()) PRINTF("{} ", k);
    std::cout << " <- LRU 顺序（key1 已被淘汰）" << std::endl;

    // ===== 9. 分组聚合（综合实战）=====
    std::cout << "--- 分组聚合 ---" << std::endl;
    const std::vector<std::pair<std::string, std::string>> staff{
        {"R&D", "张三"}, {"R&D", "李四"}, {"HR", "王五"},
    };
    std::map<std::string, std::vector<std::string>> by_dept;
    for (const auto& [dept, name] : staff) {
        by_dept[dept].push_back(name); // 对应 Collectors.groupingBy
    }
    for (const auto& [dept, names] : by_dept) {
        PRINT("{}: {} 人", dept, names.size());
    }
}
