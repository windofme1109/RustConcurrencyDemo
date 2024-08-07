use std::thread;

pub fn start_one_thread() {
    let handler = thread::spawn(|| {
        println!("Hello from a thread!");

        200
    });

    // 调用 join 方法等待线程完成执行
    // handler.join().unwrap();
    // 获取线程执行的结果
    match handler.join() {
        Ok(v) => {
            println!("thread result {}", v);
        }
        Err(err) => {
            println!("thread result {:?}", err);
        }
    }
}

pub fn start_two_thread() {
    let handler1 = thread::spawn(|| {
        println!("Hello from a thread1!");
    });

    let handler2 = thread::spawn(|| {
        println!("Hello from a thread2!");
    });

    // 调用 join 方法等待线程完成执行
    handler1.join().unwrap();

    handler2.join().unwrap();
}

pub fn start_n_threads() {
    const N: isize = 10;
    // 生成 10 个线程，并传入 handles 数组中
    let handles: Vec<_> = (0..N)
        .map(|i| {
            thread::spawn(move || {
                println!("Hello from a thread{}!", i);
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap();
    }
}