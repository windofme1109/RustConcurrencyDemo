use futures::channel::mpsc;
use futures::executor::{block_on};

// 使用 ThreadPool 之前需要启用 futures 的 thread-pool 特性
use futures::executor::ThreadPool;


use futures::StreamExt;
// futures 库 futures 是 Rust 异步编程的基础抽象库, 为编写异步代码提供了核心的 trait 和类型

// 主要提供了以下功能:
// • Future trait - 表示一个异步计算的抽象, 可以 .await 获取其结果。
// • Stream trait - 表示一个异步的数据流, 可以通过 .await 迭代获取其元素。
// • Sink trait - 代表一个可以异步接收数据的目标。
// • Executor - 执行 futures 的运行时环境。
// • Utilities - 一些组合、创建 futures 的函数

pub fn futures_async() {

    // 创建一个线程池 pool
    // ThreadPool 是一个通用目的的线程池，用于调度执行异步任务
    // 会将任意数量的任务多路复用到固定数量的工作线程上
    let pool = ThreadPool::new().expect("Failed to build pool");

    // 创建一个无边界的通道 tx 和 rx 用来在任务间传递数据
    let (tx, rx) = mpsc::unbounded::<i32>();
    // 定义一个异步任务 fut_values
    let fut_values = async {

        // 继续定义一个 future
        let fut_tx_result = async move {
            (0..100).for_each(|v| {
                tx.unbounded_send(v).expect("Failed to send");
            })
        };

        // 先用 spawn_ok 在线程池中异步执行一个任务, 这个任务会通过通道发送 0-99 的数字
        pool.spawn_ok(fut_tx_result);

        // 然后通过 rx 用 map 创建一个 Stream, 它会将收到的数字乘 2
        // 用 collect 收集 Stream 的结果到一个 Vec
        // 注意，需要引入 StreamExt trait，UnboundedReceiver 会自动实现这个 trait，这样 rx 才能有 map 方法
        let fut_values = rx.map(|v| v * 2).collect();
        // 使用提供给此异步块的执行器等待 fut_values 完成
        fut_values.await
    };

    // block_on 在主线程中执行这个异步任务并获取结果
    // Actually execute the above future, which will invoke Future::poll and
    // subsequently chain appropriate Future::poll and methods needing executors
    // to drive all futures. Eventually fut_values will be driven to completion.
    let values: Vec<i32> = block_on(fut_values);

    println!("Values={:?}", values);
}

// 这段代码展示了 futures 和通道的组合使用 - 通过线程池并发地处理数据流。
// block_on 运行 future 而不需要显式运行时也很方便。
// futures 通过异步处理数据流, 可以实现非阻塞并发程序, 这在诸如网络服务端编程中很有用。
// 与线程相比，futures 的抽象通常更轻量和高效。