use std::thread;
// thread::scope 函数提供了创建 scoped thread 的可能性
// scoped thread 不同于上面我们创建的 thread, 它可以借用 scope 外部的非 ‘static’ 数据
// 使用 thread::scope 函数提供的 Scope 的参数，可以创建 (spawn) scoped thread
// 创建出来的 scoped thread 如果没有手工调用 join , 在这个函数返回前会自动 join 
pub fn wrong_start_threads_without_scoped() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::spawn(move || {
        println!("hello from the first scoped thread");
        dbg!(&a);
    });
    // 因为 a 已经移动到了第一个线程中，所以，第二个线程没有办法再使用 a 
    // thread::spawn(move || {
    //     println!("hello from the second scoped thread");
    //     x += a[0] + a[2];
    // });
    // println!("hello from the main thread");
    // After the scope, we can modify and access our variables again:
    // a.push(4);
    // assert_eq!(x, a.len());
}

pub fn start_scoped_threads() {
    let mut a = vec![1, 2, 3]; 
    let mut x = 0;


    // 调用了 thread::scope 函数，并使用 s 参数启动了两个 scoped thread
    // 它们使用了外部的变量 a 和 x 。因为我们对 a 只是读，对 x 只有单线程的写，所以不用考虑并发问题
    // thread::scope 返回后，两个线程已经执行完毕，所以外部的线程又可以访问变量了
    thread::scope(|s| {
        s.spawn(|| {
            println!("hello from the first scoped thread");
            // 打印并返回指定的表达式的值
            // [src/scoped_thread.rs:31:13] &a = [
            //     1,
            //     2,
            //     3,
            // ]
            dbg!(&a);
        });

        s.spawn(|| {
            println!("hello from the second scoped thread"); 
            x += a[0] + a[2];

        });

        println!("hello from the main thread");
    });
    // After the scope, we can modify and access our variables again:
    a.push(4);

    assert_eq!(x, a.len());
}

