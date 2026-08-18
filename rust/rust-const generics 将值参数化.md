传统泛型是针对类型的（如 `Vec<T>`）。而 `const generics` 允许将值作为泛型参数。这使得可以在编译阶段就确定数组的大小或其他常量属性，从而进行极致的优化。

> 当数据结构的大小在编译期固定时，使用 `const generics` 替代动态的 `Vec`，可以减少堆内存分配。

```
// 定义一个固定大小的矩阵结构体
// ROWS 和 COLS 是常量泛型参数
struct Matrix<T, const ROWS: usize, const COLS: usize> {
    data: [[T; COLS]; ROWS],
}

impl<T: Default + Copy, const ROWS: usize, const COLS: usize> Matrix<T, ROWS, COLS> {
    fn new() -> Self {
        Matrix {
            data: [[T::default(); COLS]; ROWS],
        }
    }

    fn size_info(&self) {
        println!("矩阵大小: {}x{}", ROWS, COLS);
    }
}

fn main() {
    // 创建一个 4x4 的矩阵
    let mat = Matrix::<f64, 4, 4>::new();
    mat.size_info();

    // 如果尝试将不同维度的 Matrix 赋值，编译器会直接报错
    // 保证了严格的类型和内存布局安全
}
```

> 来源：[掘金 ServBay](https://juejin.cn/post/7587020682010591259)
