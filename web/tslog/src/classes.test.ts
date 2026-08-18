import { test, expect } from "bun:test";

// class：字段、构造函数、方法
class Animal {
  name: string;
  constructor(name: string) {
    this.name = name;
  }
  speak(): string {
    return `${this.name} makes a sound`;
  }
}

test("类", () => {
  const a = new Animal("cat");
  expect(a.speak()).toBe("cat makes a sound");
});

// 继承：子类 extends 父类，可重写方法
class Dog extends Animal {
  speak(): string {
    return `${this.name} barks`;
  }
}

test("继承", () => {
  const d = new Dog("wangcai");
  expect(d.speak()).toBe("wangcai barks");
});

// 访问修饰符：public（默认）/ private / protected
// 注：private/protected 是编译期约束，运行时仍可通过 (obj as any) 绕过
class Counter {
  private count = 0;
  protected label = "counter";
  increment(): number {
    this.count += 1;
    return this.count;
  }
}

test("访问修饰符", () => {
  const c = new Counter();
  expect(c.increment()).toBe(1);
  expect(c.increment()).toBe(2);
  // c.count; // 编译报错：private 属性不可访问
});

// readonly：只读字段，初始化后不可改
class Point {
  readonly x: number;
  readonly y: number;
  constructor(x: number, y: number) {
    this.x = x;
    this.y = y;
  }
}

test("readonly", () => {
  const p = new Point(3, 4);
  expect(p.x).toBe(3);
  // p.x = 5; // 编译报错
});

// getter / setter：访问器属性
class Temperature {
  private _celsius = 0;
  get celsius(): number {
    return this._celsius;
  }
  set celsius(value: number) {
    this._celsius = value;
  }
}

test("getter/setter", () => {
  const t = new Temperature();
  t.celsius = 25;
  expect(t.celsius).toBe(25);
});

// 抽象类：不能实例化，供子类继承并实现抽象方法
abstract class Shape {
  abstract area(): number;
}

// 参数属性：在构造函数参数前加修饰符，可同时声明并赋值字段
class Circle extends Shape {
  constructor(private radius: number) {
    super();
  }
  area(): number {
    return Math.PI * this.radius ** 2;
  }
}

test("抽象类", () => {
  const c = new Circle(2);
  expect(c.area()).toBeCloseTo(Math.PI * 4);
});
