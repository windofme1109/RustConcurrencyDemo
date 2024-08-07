use std::thread::{self, sleep};
use std::time::Duration;
pub fn start_thread_with_sleep() {
    // 它thread::sleep 保证当前线程 sleep 指定的时间
    // 因为它会阻塞当前的线程，所以不要在异步 的代码中调用它
    // 如果时间设置为 0, 不同的平台处理是不一样：
    // Unix 类的平台会立 即返回，不会调用 nanosleep 系统调用，
    // Windows 平台总是会调用底层的 Sleep 系统调用
    let handle1 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(2000));
        println!("Hello from a thread3!");
    });

    let handle2 = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        println!("Hello from a thread4!")
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 如果只是想让渡出时间片，你不用设置时间为 0，而是调用 yield_now 函数即可
//

pub fn start_thread_with_yield_now() {
    let handle1 = thread::spawn(|| {
        // yield_now de 作用：
        // 协同地将时间片让给操作系统调度程序
        // 这会调用底层操作系统调度程序的让出原语，表示调用线程愿意放弃其剩余的时间片
        // 以便操作系统可以在 CPU 上调度其他线程
        thread::yield_now();
        println!("yield_now!");
    });
    let handle2 = thread::spawn(|| {
        thread::yield_now();
        println!("yield now in another thread!");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}

// 如果在休眠时间不确定的情况下，我们想让某个线程休眠，将来在某个事件发生之后， 我们再主动的唤醒它
// 那么就可以使用我们前面介绍的 park 和 unpark 方法了
// 你可以认为每个线程都有一个令牌 ( token ), 最初该令牌不存在:
// thread::park 将阻塞当前线程，直到线程的令牌可用
// 此时它以原子操作的方式使用令牌。thread::park_timeout 执行相同的操作，但允许指定阻止线程的最长时间
// 和 sleep 不同，它可以还未到超时的时候就被唤醒。

// thread.upark 方法以原子方式使令牌可用(如果尚未可用)
// 由于令牌初始不存在，unpark 会导致紧接着的 park 调用立即返回

pub fn thread_park2() {
    // 调用 park 阻塞线程执行
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });
    // 调用 unpark 恢复线程执行
    handle.thread().unpark();
    handle.join().unwrap();
}

pub fn thread_park3() {
    // 调用 park 阻塞线程执行
    let parked_thread = thread::Builder::new()
        .spawn(|| {
            println!("Parking thread");
            thread::park();
            println!("Thread unparked");
        })
        .unwrap();

    thread::sleep(Duration::from_millis(1000));
    println!("Unpark the thread");
    // 调用 unpark 恢复线程执行
    parked_thread.thread().unpark();
    parked_thread.join().unwrap();
}

pub fn thread_park4() {

    // 
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(1000));
        thread::park();
        thread::park();
        thread::park();
        println!("Hello from a park thread in case of unpark first!");
    });

    // 尝试多次调用 unpark，然后调用 park
    handle.thread().unpark();
    handle.thread().unpark();
    handle.thread().unpark();
    handle.join().unwrap();

    // 结论是不可以：
    // 因为一个线程只有一个令牌，这个令牌或者存在或者只有一个
    // 多次调用 unpark 也是针对一个令牌进行的的操作，上面的代码会导致新建的那个线程一直处于 parked 状态

}
