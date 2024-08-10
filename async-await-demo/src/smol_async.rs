use smol::block_on;

// smol 是一个超轻量级的异步运行时（async runtime）库
// 专为简化异步 Rust 代码的编写而设计。它提供了一个简洁而高效的方式来管理异步任务。
// 特性
// • 轻量级： smol 的设计目标之一是轻量级，以便快速启动和低资源开销。
// • 简洁 API： 提供简洁的 API，使得异步任务的创建、组合和运行变得直观和简单
// • 零配置： 无需复杂的配置，可以直接在现有的 Rust 项目中使用。
// • 异步 I/O 操作： 支持异步文件 I/O、网络操作等，使得异步编程更加灵活


// https://crates.io/crates/smol

pub fn smol_async() {
    block_on(hello_async());
    block_on(async { println!("Hello from smol") });
}

async fn hello_async() {
    println!("Hello async from smol");
}