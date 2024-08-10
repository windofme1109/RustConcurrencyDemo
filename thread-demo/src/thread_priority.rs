use std::convert::TryInto;
use std::thread;
use thread_priority::*;

// 通过 第三方 crate thread-priority 可以设置线程的优先级
// 因为 Rust 的线程都是纯的操作系统的优先级，现代的操作系统的线程都有优先级的概念，所以可以通过系统调用等方式设置优先级
// 唯一一点不好的就是各个操作系统的平台的优先级的数字和范围不一样。当前这个库支持以下的平台：
// • Linux
// • Android
// • DragonFly
// • FreeBSD
// • OpenBSD
// • NetBSD
// • macOS
// • Windows

pub fn start_thread_with_priority() {
    let handle1 = thread::spawn(|| {
        // 设定线程的优先级
        assert!(set_current_thread_priority(ThreadPriority::Min).is_ok());
        println!("Hello from a thread5!");
    });

    let handle2 = thread::spawn(|| {
        // 还可以通过设置数字的方式设置线程的优先级
        // 数字越低，优先级越低
        assert!(
            set_current_thread_priority(ThreadPriority::Crossplatform(0.try_into().unwrap()))
                .is_ok()
        );

        assert!(set_current_thread_priority(ThreadPriority::Max).is_ok());
        println!("Hello from a thread6!");
    });

    let handle3 = thread::spawn(|| {
        // 还可以通过设置数字的方式设置线程的优先级
        // 数字越低，优先级越低
        assert!(
            set_current_thread_priority(ThreadPriority::Crossplatform(0.try_into().unwrap()))
                .is_ok()
        );

        println!("Hello from a thread3!")
    });

    // 你还可以设置特定平台的优先级值，以 windows 平台为例：
    // assert! (set_current_thread_priority(ThreadPriority::Os(
    //     WinAPIThreadPriority::Lowest.into())). is_ok());
    // }

    handle1.join().unwrap();
    handle2.join().unwrap();
    handle3.join().unwrap();
}

pub fn thread_builder() {

    // thread_priority 还提供了一个 ThreadBuilder, 类似标准库的 ThreadBuilder, 只不过增加设置优先级的能力

    // 通过 priority 函数设置线程的优先级
    let thread1 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn(|result| {
            println!("Set priority result: {:?}", result);
            assert!(result.is_ok());
        })
        .unwrap();

    let thread2 = ThreadBuilder::default()
        .name("MyThread")
        .priority(ThreadPriority::Max)
        .spawn_careless(|| {
            // 设置 affinity
            println!("We don't care about the priority result.");
        })
        .unwrap();
    // 或者使用 thread_priority::ThreadBuilderExt; 扩展标准库的 ThreadBuilder 支持设置优先级


    thread1.join().unwrap();
    thread2.join().unwrap();

    // 获取当前线程的优先级
    assert!(thread::current().get_priority().is_ok());
    assert!(thread::current().get_priority().is_ok());
    println!("This thread's native id is: {:?}", thread::current().get_native_id());
    println!("This thread's native id is: {:?}", thread::current().get_native_id().unwrap());
}
