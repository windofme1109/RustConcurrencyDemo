use std::thread;
use std::time::Duration;

// 获取当前的线程
// 因为线程是操作系统最小的调度和运算单元，所以一段代码的执行隶属于某个线程
// 如何获得当前的线程呢?通过 thread::current() 就可以获得，它会返回一个 Thread 对象
// 你可以通过它获得线程的 ID 和 name


pub fn current_thread() {
    let current_thread = thread::current();
    println!("current thread: {:?},{:?}", current_thread.id(), current_thread.name());

    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

    let handler = builder.spawn(|| {
        let current_thread = thread::current(); 
        println!("child thread: {:?},{:?}", current_thread.id(), current_thread.name());
    }).unwrap();

    handler.join().unwrap();

}


// 使用 unpark 方法，唤醒被阻塞（parked）的线程

pub fn unpark_thread() {
    let parked_thread = thread::Builder::new().spawn(|| {
        println!("Parking thread");
        // 手动阻塞线程
        thread::park();

        println!("Thread unparked");
    }).unwrap();

    // 延迟 10 ms
    thread::sleep(Duration::from_millis(1000));

    println!("Unparked the thread");

    // 唤醒线程
    parked_thread.thread().unpark();

    parked_thread.join().unwrap();
}