use std::thread;

// builder 构建线程
pub fn start_one_thread_by_builder() {
    // 设置线程名称和占用堆栈大小
    let builder = thread::Builder::new()
        .name("foo".into())
        .stack_size(32 * 1024);

        // 使用 spawn 方法开启一个线程
    let handler = builder
        .spawn(|| {
           
            println!("Hello from a thread!");

        })
        .unwrap();

   
    handler.join().unwrap();

}
