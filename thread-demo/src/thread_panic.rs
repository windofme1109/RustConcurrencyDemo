use std::thread;
use std::time::Duration;
use std::panic::catch_unwind;



// 线程异常处理 - panic

// Rust 中致命的逻辑错误会导致线程 panic, 出现 panic 是线程会执行栈回退
// 运行解构器以及释放拥有的资源等等。 Rust 可以使用 catch_unwind 实现类似 try/catch 捕获panic 的功能或者 resume_unwind 继续执行
// 如果 panic 没有被捕获，那么线程就会退出，通过 JoinHandle 可以检查

pub fn panic_example() {
    println!("Hello, world!");

    let h = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        panic!("boom");
    });

    let r = h.join();

    // 线程执行出现 panic，join 会返回 Err 变体
    // 手动处理 panic
    match r {
        Ok(r) => println!("All is well! {:?}", r),
        Err(e) => println!("Got an error! {:?}", e),
    }

    println!("Exiting main!");
}

pub fn panic_caught_example() {
    println!("Hello, panic_caught_example !");

    let h = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));

        // panic 如果被捕获，外部的 handle 是检查不到这个 panic 的
        let result = catch_unwind(|| {
            panic!("boom");
        });
        // true
        println!("panic caught, result = {}", result.is_err());
    });

    let r = h.join();

    match r {
        // here
        Ok(r) => println!("All is well! {:?}", r),
        // 外界获取不到 panic
        Err(e) => println!("Got an error! {:?}", e),
    }
    println!("Exiting main!")
}


// 通过 scope 生成的 scope thread，任何一个线程 panic, 如果未被捕获，那么 scope 返回就是这个错误
pub fn thread_scope_panic_example() {
    thread::scope(|s| {
       let h1 = s.spawn(|| {
            thread::sleep(Duration::from_millis(1000));
            panic!("scope boom");
        });
        // let r = h1.join();
        // match r {
        //     Ok(res) => println!("res {:?}", res),
        //     Err(err) => println!("Got an error! {:?}", err),
        // }

    });




}