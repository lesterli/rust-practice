# Rust实战

## 并发

创建一个新线程，调用`thread::spawn`函数并传递一个闭包，其重包含希望在新线程运行的代码。

一个确保安全并发的方式是消息传递（message passing），线程通过发送包含数据的消息来相互通信。

> “Do not communicate by sharing memory; instead, share memory by communicating.”

通道（channel）来实现消息传递，创建通道，使用`mpsc::channel`函数。

不允许在线程间直接传递引用，那如何在多个线程间安全地共享变量呢？

不可变的变量，通过`Arc<T>`来共享，它是`Rc<T>`的线程安全版本，其内部使用了原子操作。

有两个并发概念是内嵌于语言中的：`std::marker`中的`Sync`和`Send` trait，使得并发保证能被扩展到用户定义的和标准库中提供的类型中。

## 代码实现

* [并发](./std-marker/example_Send_Sync.rs)

