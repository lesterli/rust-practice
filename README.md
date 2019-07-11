# Rust语言学习笔记

## [Rust概述](https://mp.weixin.qq.com/s?__biz=MzA5NTQyNDIyNw==&mid=2247483774&idx=1&sn=0f76ebdaf945da8f28c7753d9fd4399a)

摘录片段：
Rust代码非常接近自然语言。

```Rust
5.times(|| println!("Hello Rust"));
2.days().from_now();
```
## [Rust基本语法](https://mp.weixin.qq.com/s?__biz=MzA5NTQyNDIyNw==&mid=2247483778&idx=1&sn=52c655fb8bbc81eaaa0ce2acc1c9d07f)

Rust一切皆表达式。

个人觉得闭包降低了代码的可读性，一段好的代码，要有良好的阅读体验，而不是写的时候方便。

### 练习代码

* [Rust表达式](https://github.com/lesterli/rust-practice/blob/master/src/statement.rs)
* [Rust变量](https://github.com/lesterli/rust-practice/blob/master/src/variable.rs)
* [Rust函数](https://github.com/lesterli/rust-practice/blob/master/src/function.rs)
* [Rust流程控制](https://github.com/lesterli/rust-practice/blob/master/src/control_flow.rs)

## [Rust数据类型](https://mp.weixin.qq.com/s?__biz=MzA5NTQyNDIyNw==&mid=2247483783&idx=1&sn=8625be095bdcc56758136cd837e5b107)

利用元组，函数可以返回多个值。

什么是字符串？

Rust原始的字符串类型：str，它是固定长度的。

字符串切片slice：&str，它是储存在别处的UTF-8编码字符串数据的引用。

而称作String类型的字符串是由标准库提供的，它是可增长的、可变长度的、有所有权的、UTF-8编码的字符串类型。

### 练习代码

* [Rust原始类型](https://github.com/lesterli/rust-practice/blob/master/src/primitives.rs)
* [Rust集合类型](https://github.com/lesterli/rust-practice/blob/master/src/collections.rs)
