use futures_lite::future;
// futures-lite 这个库是 futures 的一个子集：
// 它的编译速度快了一个数量级, 修复了 futures API 中的一些小问题, 补充了一些明显的空白, 并移除了绝大部分不安全的代码。
// 简而言之, 这个库的目标是比 futures 更可易用, 同时仍然与其完全兼容。
// https://crates.io/crates/futures-lite
pub async fn hello_async() {
    println!("Hello, async world");
}


pub fn run_hello_async() {
    // block_on 运行异步任务
    future::block_on(hello_async());
}
