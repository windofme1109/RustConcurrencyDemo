use crossbeam::thread;

// crossbeam 也提供了创建了 scoped thread 的功能
// 和标准库的 scope 功能类似，但是它创建的 scoped thread 可以继续创建 scoped thread

// https://crates.io/crates/crossbeam

pub fn crossbeam_scope() {
    let mut a = vec![1, 2, 3];
    let mut x = 0;
    thread::scope(|s| {
        // 这里我们创建了两个子线程，子线程在 spawn 的时候，传递了一个 scope 值的
        // 利用这个 scope 值，还可以在子线程中创建孙线程
        s.spawn(|another_scope| {
            another_scope.spawn(|_| {
                println!("child thread vec a is {:?}", &a);
            });
        });

        s.spawn(|_| {
            println!("hello from the first crossbeam scoped thread");
            // dbg!(&a);
        });
        s.spawn(|_| {
            println!("hello from the second crossbeam scoped thread");
            x += a[0] + a[2];
        });
        println!("hello from the main thread");
    })
    .unwrap();

    // After the scope, we can modify and access our variables again:
    a.push(4);
    assert_eq!(x, a.len());

    println!("vec a is {:?}", a);
}


