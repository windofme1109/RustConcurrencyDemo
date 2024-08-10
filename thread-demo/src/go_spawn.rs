
use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::Arc;

use go_spawn::{go, join};

// Go 风格的启动线程
// 你了解过 Go 语言吗？如果你稍微看过 Go 语言，就会发现它的开启新的 goroutine 的方法非常的简洁：
// 通过 go func() {...}() 就启动了一个 goroutine，貌似同步的代码，却是异步的执行
// 有一个第三方的库 go-spawn，可以提供 Go 类似的便利的方法：
pub fn go_thread() {
    // AtomicI64 创建一个可以在线程之间安全分享的整数类型
    let counter = Arc::new(AtomicI64::new(0));

    let counter_cloned = counter.clone();

    // 使用 go! 或者 go_ref! 启动一个线程
    go! {
        for _ in 0..100 {
            // 将指定的值加到当前值（counter_cloned）中
            counter_cloned.fetch_add(1, Ordering::SeqCst);
        }
    }

    // 使用 join! 或者 join_all! 把最近 go_spawn 创建的线程 join 起来
    // Join the most recent thread spawned by `go_spawn` that has not yet been joined
    assert!(join!().is_ok());

    assert_eq!(counter.load(Ordering::SeqCst), 100);

    println!("counter: {:?}", counter);
}
