use std::cell::{Cell, RefCell};
use std::thread;

pub fn start_threads_with_threadlocal() {
    
    // ThreadLocal 为 Rust 程序提供 thread-local storage 的实现
    // TLS(thread-local storage) 可以存储数据到全局变量中，每个线程都有这个存储变量的副本
    // 线程不会分享这个数据，副本是线程独有的，所以对它的访问不需要 同步控制

    // 使用 thread_local! 宏去声明任意数量的静态变量
    // 每个线程都会有这个静态变量的副本

    // thread-local key 拥有它的值，并且在线程退出此值会被销毁

    thread_local! {
        // 只能获得对内部数据的共享引用(&T)，因此通常使用 Cell 或 RefCell 之类的类型来允许可变访问
        static COUNTER: RefCell<u32> = RefCell::new(1);
    };

    // 使用 with 访问函数去访问静态变量的值
    COUNTER.with(|c| {
        *c.borrow_mut() = 2;
    });

    let handle1 = thread::spawn(move || { 

        // 在线程内修改 COUNTER，只能影响本线程的 COUNTER 数据，不会影响外面的 COUNTER

        COUNTER.with(|c| {
            *c.borrow_mut() = 3;
        });
    });

    COUNTER.with(|c| {
        println!("Hello from a thread7, c={}!", *c.borrow());
    });

    let handle2 = thread::spawn(move || { 
        COUNTER.with(|c| {
            *c.borrow_mut() = 4;
        });
        COUNTER.with(|c| {
            println!("Hello from a thread8, c={}!", *c.borrow());
        });
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    COUNTER.with(|c| {
        println!("Hello from main, c={}!", *c.borrow());
    });
}