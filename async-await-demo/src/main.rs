use std::future::Future;
// use std::future::*;
use futures::join;
use futures::executor::block_on;


mod tokio_async;
mod futures_async;
mod futures_lite_async;
mod async_std_demo;
mod smol_async;
mod join_select;

use tokio_async::*;

use futures_async::*;
use crate::async_std_demo::*;
use crate::futures_lite_async::*;
use crate::join_select::*;
use crate::smol_async::*;
// 异步编程具有以下优势：
// • 提高系统的并发能力和响应速度
// • 减少线程等待时间，提高资源利用率
// • 可以处理大量的并发请求或任务
// • 支持高效的事件驱动编程风格

// 异步编程广泛应用于以下场景：
// • 网络编程：处理大量的并发网络请求
// • I/O 密集型任务：如文件操作、数据库访问等
// • 用户界面和图形渲染：保持用户界面的流畅响应
// • 并行计算：加速复杂计算任务的执行


// Tokio 的主要组件包括:
// • tokio - 核心运行时, 提供任务调度,IO 资源等。
// • tokio::net - 异步 TCP、 UDP 的实现。
// • tokio::sync - 互斥量、信号量等并发原语。
// • tokio::time - 时间相关工具。
// • tokio::fs - 异步文件 IO。


fn main() {
    // tokio_async();

    // futures_async();

    // run_hello_async();

    // run_hello_async_from_async_std();

    // smol_async();

    // run_async_fun();
    smol_zip()
}



// 使用 tokio::main 宏标记异步函数
// 该异步函数会被选定运行时执行
// #[tokio::main]
// async fn main() {
//
//
//     // 运行时中异步执行任务
//     tokio::spawn(async {
//         // do work
//         println!("tokio async");
//     });
//
//     executor_async_func().await;
//
//     // 目前还没有办法为 async 语句块指定返回类型
//     // 编译器也无法推断出代码块返回值的类型，需要我们给返回值添加类型注释
//     // let fut = async {
//     //     foo().await?;
//     //     bar().await?;
//     //     // 显示添加类型
//     //     Ok::<(), String>(())
//     // };
//     // println!("Hello, world!");
//
//     // 在异步函数中调用另外的异步函数，使用 .await 即可推动被调用的异步任务开始执行
//     // 在非异步函数中，需要一个执行器推动异步开始轮询
//     // futures trait 提供的 block_on 方法就是一个执行器，它会阻塞当前的线程，直到一个异步任务完成
//     // block_on(executor_async_func());
//
// }


async fn executor_async_func() {

    // futures trait 中的 join 宏必须在一个异步代码中使用
    // 作用是：同时轮询多个 future，完成后返回所有结果的元组
    let res = join!(foo(), foo_2());

    println!("{:?}", res.0);
}

// `foo()` 返回 `Future<Output = u8>`
// 当调用 foo().await 时， 该 Future 会被执行，调用结束以后，将获得一个 u8 值
async fn foo() -> u8 {
    5
}
fn bar() -> impl Future<Output=u8> {
    // 下面的 async 语法块返回 Future<Output = u8>
    async {
        // async 是懒惰的，直到被执行器 poll 或者 .await 后才会开始运行其中后者是最常用的运行 Future 的方法
        // 当 .await 被调用时，它会尝试运行 Future 直到完成，但是若该 Future 进入阻塞，那就会让出当前线程的控制权
        let x: u8 = foo().await;
        x + 5
    }
}

async fn foo_2() -> Result<u8, String> {
    Ok(1)
}
async fn bar_2() -> Result<u8, String> {
    Ok(1)
}

