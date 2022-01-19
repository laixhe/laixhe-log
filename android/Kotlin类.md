#### 普通类
```
class Person {
    fun test(){
        println("Person.test")
    }
}
# 调用：Person().test()
```

#### 静态类
> 静态类里面的方法，都是静态方法
```
object Person {
    fun test(){
        println("Person::test")
    }
}
# 调用：Person.test()
```

#### 伴生类的方法
> 在 Kotlin 中并没有 static 关键字，不过可借助 companion object 来实现类静态方法的目的
```
class Person {
    
    // 里面定义都是静态方法
    companion object {
        fun test1(){
            println("Person::test1")
        }
    }
    
    fun test2(){
        println("Person.test2")
    }
}

# 调用：Person.test1()
# 调用：Person().test2()
```
