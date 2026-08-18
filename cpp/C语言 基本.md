# C 语言基本数据类型

C 语言的数据类型分为「基本类型」「构造类型」「指针类型」「空类型」四大类。

## 类型总览

```
基本类型  整型     short / int / long / long long（signed / unsigned）
         字符型    char
         浮点型    float / double
         枚举型    enum
构造类型  数组
         结构体    struct
         共用体    union
指针类型  （任意类型 *）
空类型    void
逻辑类型  bool     true(非零) / false(零)
```

## 基本类型

### 整型

```c
#include <stdio.h>

int main() {
    short s = 10;           // 短整型
    int i = 20;             // 整型（最常用）
    long l = 30L;           // 长整型
    long long ll = 40LL;    // 长长整型

    unsigned int u = 50U;   // 无符号整型（只能存非负数）

    printf("short=%hd int=%d long=%ld longlong=%lld unsigned=%u\n",
           s, i, l, ll, u);
    return 0;
}
```

### 字符型

```c
char c = 'A';               // 占 1 字节，本质是整数
printf("%c = %d\n", c, c);  // A = 65（ASCII 码）
```

### 浮点型

```c
float f = 3.14f;             // 单精度，约 6~7 位有效数字
double d = 3.14159265358979; // 双精度，约 15~16 位有效数字
printf("%f %lf\n", f, d);
```

## 构造类型

### 数组

```c
int arr[5] = {1, 2, 3, 4, 5};   // 固定长度的同类型元素集合
printf("%d\n", arr[0]);         // 下标从 0 开始
```

### 结构体（struct）

```c
struct Point {
    int x;
    int y;
};

struct Point p = {10, 20};
printf("(%d, %d)\n", p.x, p.y);
```

### 共用体（union）

```c
// 所有成员共享同一块内存，只能同时使用一个成员
union Value {
    int i;
    float f;
};

union Value v;
v.i = 100;
printf("%d\n", v.i);        // 100
```

## 指针类型

```c
int a = 10;
int *p = &a;                // p 存的是 a 的地址
printf("%d\n", *p);         // 解引用：取出 p 指向的值 → 10
```

## 空类型（void）

```c
void say() {                // 无返回值的函数
    printf("hello\n");
}
```

## 逻辑类型（bool）

```c
#include <stdbool.h>

bool ok = true;             // 非零为真，零为假
if (ok) {
    printf("true\n");
}
```
