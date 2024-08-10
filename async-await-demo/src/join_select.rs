// use std::io::Error;
use futures::future::{FutureExt};
use futures::{try_join, join, select};

use futures::future;
use futures::executor::block_on;



// 在 Rust 中, 有两个常见的宏可以用于同时等待多个 future
// select 宏
// join 宏

// select!
// select! 宏可以同时等待多个 future, 并只处理最先完成的那个 future:
pub async fn async_select_demo() {

    let mut a = future::ready(4);
    let mut b = future::pending::<()>();

    let result = select! {
        // 如果异步任务不是由异步函数生成，可以不需要使用 fuse 函数执行异步函数
        a_res = a => a_res + 1,
        _b = b => 0
    };

    assert_eq!(result, 5);
    println!("{}", result);
}

pub async fn async_select_demo_2() {
    let res = select! {
        // Calling the following async fn returns a Future which does not
        // implement Unpin
        a = hello_1().fuse() => {
            // 处理异步任务返回值

            // hello_1().fuse() 相当于执行异步函数，并使用变量 a 接收异步结果
            // 在这里就可以使用变量 a
            println!("{}", a);
        },
        b = hello_2().fuse() => {
            println!("{}", b);
        }
    };



}
async fn hello_1() -> String {
    "hello select 1".to_string()
}

async fn hello_2() -> String {
    "hello select 2".to_string()
}

pub fn run_async_fun() {
    block_on(async_select_demo());
    block_on(async_select_demo_2());
    block_on(async_join_demo());
    block_on(async_try_join_demo());
}


// join!
// join! 宏可以同时等待多个 future, 并处理所有 future
pub async fn async_join_demo() {

    let future1 = async {
        "future 1"
    };
    let future2 = async {
        "future 2"
    };
    let (res1, res2) = join!(future1, future2);

    println!("res1 {}, res2 {}", res1, res2);
}


// try_join!
// try_join! 宏也可以用于同时等待多个 future, 它与 join! 类似, 但是有一点不同:
// try_join! 在任何一个 future 返回错误时, 就会提前返回错误, 而不会等待其他 future
pub async fn async_try_join_demo() {
    let future1 = async {
        Ok::<(), Error>(())
    };

    // 让 future2 这个异步任务返回 Err
    let future2 = async {
        Err::<(), Error>(Error::SomethingBad)
    };

    let result = try_join!(future1, future2);

    println!("{:?}", result);
}

// 所以 try_join! 的用途是同时启动多个 future, 但是遇到任何一个错误就立即返回
// 避免不必要的等待。这在需要并发但不能容忍任何失败的场景很有用

// 而当需要等待所有 future 无论成功失败, 获取所有结果的时候, 再使用 join!

// zip
// zip 函数会 join 两个 future, 并等待他们完成
// try_zip 函数会 join 两个函数，但是会等待两个 future 都完成或者其中一个 Err 则返回
pub fn smol_zip() {
    smol::block_on(async {
        use smol::future::{try_zip, zip, FutureExt};

        let future1 = async { 1 };
        let future2 = async { 2 };
        // 等待两个异步任务完成，并返回所以的结果
        let result = zip(future1, future2);

        println!("smol_zip: {:?}", result.await);

        let future1 = async { Ok::<i32, i32>(1) };
        let future2 = async { Err::<i32, i32>(2) };
        // 等待两个异步任务都完成，或者有一个异步任务返回 Err
        let result = try_zip(future1, future2).await;

        println!("smol_try_zip: {:?}", result);
    });
}

#[derive(Debug)]
enum Error {
    SomethingBad
}
