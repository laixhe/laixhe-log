package com.laixhe.javalog;

import com.laixhe.javalog.demo.StreamDemo;
import org.junit.jupiter.api.Test;

import java.util.List;
import java.util.Map;
import java.util.stream.Collectors;
import java.util.stream.IntStream;

import static org.junit.jupiter.api.Assertions.assertEquals;
import static org.junit.jupiter.api.Assertions.assertTrue;

/**
 * 流/迭代器测试（对应 Rust iterators.rs 的 #[cfg(test)] 练习题）。
 */
class StreamTest {

    // 练习 1：偶数平方后求和（1..=5 → 2^2 + 4^2 = 20）
    @Test
    void exercise1_even_squares_sum() {
        int sum = IntStream.rangeClosed(1, 5)
                .filter(x -> x % 2 == 0)
                .map(x -> x * x)
                .sum();
        assertEquals(20, sum);
    }

    // 练习 2：enumerate 与 zip 的区别
    @Test
    void exercise2_enumerate_zip() {
        // enumerate：索引固定从 0 开始（用 IntStream.range 模拟）
        List<String> e = IntStream.range(0, 3)
                .mapToObj(i -> i + ":'" + "abc".charAt(i) + "'")
                .toList();
        assertEquals(List.of("0:'a'", "1:'b'", "2:'c'"), e);

        // zip：可以和任意序列配对，这里是 100 开头的序列
        List<String> z = IntStream.range(0, 3)
                .mapToObj(i -> (100 + i) + ":'" + "abc".charAt(i) + "'")
                .toList();
        assertEquals(List.of("100:'a'", "101:'b'", "102:'c'"), z);
    }

    // 练习 3：flatMap 把每个单词展开成字符
    @Test
    void exercise3_flat_map_chars() {
        List<Character> chars = List.of("hello", "world").stream()
                .flatMap(w -> w.chars().mapToObj(c -> (char) c))
                .toList();
        assertEquals(List.of('h', 'e', 'l', 'l', 'o', 'w', 'o', 'r', 'l', 'd'), chars);
    }

    // 练习 4：综合实战 —— R&D 部门 30 岁以上员工平均月薪
    @Test
    void exercise4_practice() {
        record Employee(String dept, int age, int salary) {
        }
        List<Employee> staff = List.of(
                new Employee("R&D", 28, 30000),
                new Employee("R&D", 35, 45000),
                new Employee("R&D", 42, 60000),
                new Employee("HR", 32, 18000),
                new Employee("R&D", 25, 22000),
                new Employee("Sale", 38, 25000));

        double avg = staff.stream()
                .filter(e -> e.dept().equals("R&D"))
                .filter(e -> e.age() >= 30)
                .mapToInt(Employee::salary)
                .average()
                .orElse(0);
        assertEquals(52500.0, avg, 0.001); // (45000+60000)/2 = 52500
    }

    // 练习 5：partition 奇偶分组（对应 Rust partition 消费器）
    @Test
    void exercise5_partition() {
        List<Integer> v = List.of(3, 1, 4, 1, 5, 9, 2, 6);
        Map<Boolean, List<Integer>> partition = v.stream()
                .collect(Collectors.partitioningBy(x -> x % 2 == 0));
        assertEquals(List.of(4, 2, 6), partition.get(true));
        assertEquals(List.of(3, 1, 1, 5, 9), partition.get(false));
    }

    // 运行完整 Demo
    @Test
    void runStreamDemo() {
        StreamDemo.basics();
        StreamDemo.adapters();
        StreamDemo.consumers();
        StreamDemo.practice();
        assertTrue(true);
    }
}
