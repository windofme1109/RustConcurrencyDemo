use async_std::task;

// async-std 是一个为 Rust 提供异步标准库的库。它扩展了标准库，使得在异步上下文
// 中进行文件 I/O、网络操作和任务管理等操作更加便捷。
// 它提供了你所习惯的所有接口, 但以异步的形式, 并且准备好用于 Rust 的 async/await 语法。
// 特性
// • 现代: 从零开始针对 std::future 和 async/await 构建, 编译速度极快。
// • 快速: 我们可靠的分配器和线程池设计提供了超高吞吐量和可预测的低延迟。
// • 直观: 与标准库完全对等意味着你只需要学习一次 API。
// • 清晰: 详细的文档和可访问的指南意味着使用异步 Rust 从未如此简单

// https://crates.io/crates/async-std

async fn hello_async() {
    println!("Hello, async world!");
}


pub fn run_hello_async_from_async_std() {

    // 使用 task::block_on 来执行这个异步函数。 block_on 会阻塞当前线程，直到传入的 future 运行完成
    task::block_on(hello_async());
}
