package com.laixhe.javalog.demo;

import java.util.*;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

/**
 * 迭代器/流专题：Java Stream 是「惰性求值的元素序列」，对应 Rust 迭代器。
 * 对应 Rust rustlog/src/iterators.rs。
 *
 * 两类操作：
 * - 中间操作（Adapter）：map / filter / limit / skip / flatMap 等，惰性执行，返回新流
 * - 终结操作（Consumer）：collect / sum / count / reduce / forEach 等，真正触发计算
 *
 * 对应关系速查：
 * - map → map / mapToInt；filter → filter；filter_map → filter+map / flatMap(Optional::stream)
 * - take(n) → limit(n)；skip(n) → skip(n)；step_by → IntStream.iterate
 * - enumerate → IntStream.range + 索引；zip → 两个流配对；chain → Stream.concat
 * - flatten → flatMap；fold/reduce → reduce；partition → partitioningBy
 */
public final class StreamDemo {

    private StreamDemo() {
    }

    // ============ 基础：流的三种创建方式 ============
    public static void basics() {
        // 1) 从集合创建（对应 iter()，只读借用）
        List<Integer> v = List.of(10, 20, 30);
        System.out.print("stream(): ");
        v.stream().forEach(x -> System.out.print(x + " "));
        System.out.println();

        // 2) 从数组创建
        int[] arr = {1, 2, 3};
        int sum = Arrays.stream(arr).sum();
        System.out.println("Arrays.stream 求和: " + sum); // 6

        // 3) 范围创建（对应 Range 迭代器 1..=10，包含两端）
        int total = IntStream.rangeClosed(1, 10).sum();
        System.out.println("rangeClosed(1,10) 求和: " + total); // 55

        // 手动迭代（对应手动调用 next()，展示迭代器底层）
        Iterator<Integer> it = v.iterator();
        System.out.println("手动 next: " + it.next() + " " + it.next() + " " + it.next() + " " + it.hasNext());
        // 10 20 30 false
    }

    // ============ 中间操作（惰性！）============
    public static void adapters() {
        // ---- map：对每个元素做变换 ----
        List<Integer> squares = IntStream.rangeClosed(1, 10).map(x -> x * x).boxed().toList();
        System.out.println("1..=10 平方: " + squares); // [1,4,9,16,25,36,49,64,81,100]

        // ---- filter：只保留满足条件的元素 ----
        List<String> words = List.of("rust", "go", "python", "java", "c++", "js");
        List<String> shortWords = words.stream().filter(w -> w.length() <= 3).toList();
        System.out.println("长度≤3 的单词: " + shortWords); // [go, c++, js]

        // ---- filter + map 二合一（对应 filter_map：把能解析为数字的挑出来）----
        List<String> strs = List.of("123", "abc", "456", "not_a_num", "789");
        List<Integer> nums = strs.stream()
                .map(s -> {
                    try {
                        return Optional.of(Integer.parseInt(s));
                    } catch (NumberFormatException e) {
                        return Optional.<Integer>empty();
                    }
                })
                .flatMap(Optional::stream) // 对应 filter_map
                .toList();
        System.out.println("filter_map 选出合法数字: " + nums); // [123, 456, 789]

        // ---- take(n) / skip(n)：取前 n 个 / 跳过前 n 个 ----
        System.out.println("limit(3): " + IntStream.rangeClosed(1, 10).limit(3).boxed().toList()); // [1,2,3]
        System.out.println("skip(7): " + IntStream.rangeClosed(1, 10).skip(7).boxed().toList());   // [8,9,10]

        // ---- step_by(n)：每 n 个取一个（对应 step_by(5) 0..=20）----
        List<Integer> stepped = IntStream.iterate(0, i -> i <= 20, i -> i + 5).boxed().toList();
        System.out.println("step_by(5) 0..=20: " + stepped); // [0,5,10,15,20]

        // ---- enumerate：给每个元素加上索引（对应 enumerate）----
        IntStream.range(0, "Rust".length())
                .forEach(i -> System.out.println("  enumerate: [" + i + "] = '" + "Rust".charAt(i) + "'"));
        // [0]='R' [1]='u' [2]='s' [3]='t'

        // ---- zip：把两个流的元素一一配对（长度以较短为准）----
        List<String> names = List.of("Alice", "Bob", "Charlie");
        List<Integer> scores = List.of(95, 87, 92);
        List<String> pairs = IntStream.range(0, Math.min(names.size(), scores.size()))
                .mapToObj(i -> "(" + names.get(i) + "," + scores.get(i) + ")")
                .toList();
        System.out.println("zip 配对: " + pairs); // [(Alice,95), (Bob,87), (Charlie,92)]

        // ---- chain：把两个流首尾相接（对应 chain）----
        List<Integer> chained = IntStream.concat(
                IntStream.rangeClosed(1, 3),
                IntStream.rangeClosed(10, 12)).boxed().toList();
        System.out.println("chain: " + chained); // [1,2,3,10,11,12]

        // ---- flatMap：把嵌套的流展平一层（对应 flatten / flat_map）----
        List<List<Integer>> nested = List.of(List.of(1, 2), List.of(3, 4, 5), List.of(6));
        List<Integer> flat = nested.stream().flatMap(List::stream).toList();
        System.out.println("flatMap 展平: " + flat); // [1,2,3,4,5,6]

        // 把每个单词的字符展开（对应 flat_map chars）
        List<Character> chars = List.of("hello", "world").stream()
                .flatMap(w -> w.chars().mapToObj(c -> (char) c))
                .toList();
        System.out.println("flat_map 展开字符: " + chars); // [h,e,l,l,o,w,o,r,l,d]
    }

    // ============ 终结操作（真正触发计算）============
    public static void consumers() {
        List<Integer> v = List.of(3, 1, 4, 1, 5, 9, 2, 6);

        // ---- collect：收集到目标集合（对应 collect）----
        List<Integer> doubled = v.stream().map(x -> x * 2).toList();
        System.out.println("collect 到 List: " + doubled);

        Set<Integer> unique = v.stream().collect(Collectors.toSet());
        System.out.println("collect 到 Set（去重）: " + unique);

        // ---- sum / count ----
        int sum = v.stream().mapToInt(Integer::intValue).sum();
        long count = v.stream().count();
        System.out.println("sum=" + sum + ", count=" + count); // sum=31, count=8

        // ---- min / max（返回 Optional，空流是 empty）----
        System.out.println("min=" + v.stream().min(Integer::compareTo)
                + " max=" + v.stream().max(Integer::compareTo)); // Optional[1] Optional[9]

        // ---- any / all：是否「有一个」/「全部」满足条件 ----
        System.out.println("any > 10? " + v.stream().anyMatch(x -> x > 10)); // false
        System.out.println("all > 0?  " + v.stream().allMatch(x -> x > 0));  // true

        // ---- forEach：对每个元素执行副作用（对应 for_each）----
        System.out.print("forEach: ");
        IntStream.rangeClosed(1, 5).forEach(x -> System.out.print(x + " "));
        System.out.println();

        // ---- reduce / fold：累积聚合（对应 fold / reduce）----
        int foldSum = IntStream.rangeClosed(1, 10).reduce(0, Integer::sum); // 带初始值
        System.out.println("reduce 累加 1..=10 = " + foldSum); // 55

        Optional<Integer> maxReduce = v.stream().reduce(Integer::max);
        System.out.println("reduce 手动求最大值 = " + maxReduce); // Optional[9]

        // ---- partition：按条件分成两组（对应 partition）----
        Map<Boolean, List<Integer>> partition = v.stream()
                .collect(Collectors.partitioningBy(x -> x % 2 == 0));
        System.out.println("partition 奇偶分: 偶=" + partition.get(true) + "  奇=" + partition.get(false));
    }

    // ============ 综合实战：用流处理复杂查询 ============
    // 场景：给定一批员工（部门，年龄，月薪），求出「R&D 部门 30 岁以上员工的平均月薪」。
    public static void practice() {
        record Employee(String dept, int age, int salary) {
        }

        List<Employee> staff = List.of(
                new Employee("R&D", 28, 30000),
                new Employee("R&D", 35, 45000),
                new Employee("R&D", 42, 60000),
                new Employee("HR", 32, 18000),
                new Employee("R&D", 25, 22000),
                new Employee("Sale", 38, 25000));

        // 要求：R&D 部门 + 30 岁以上 → 平均月薪
        var query = staff.stream()
                .filter(e -> e.dept().equals("R&D"))   // 先筛选部门
                .filter(e -> e.age() >= 30)            // 再筛选年龄
                .mapToInt(Employee::salary)            // 提取月薪
                .toArray();

        double avg = query.length == 0 ? 0.0 : Arrays.stream(query).average().orElse(0);
        System.out.printf("R&D 30+ 员工平均月薪: %.0f 元/月%n", avg); // (45000+60000)/2 = 52500
    }
}
