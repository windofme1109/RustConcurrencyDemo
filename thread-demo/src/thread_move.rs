use std::thread;

// move

pub fn start_one_thread_with_move() {
    let x = 100;

    // 使不使用 move 依赖相应的闭包是否要获取外部变量的所有权:
    // 如果不获取外部变量的所有权，则可以不使用 move
    // 大部分情况下我们会使用外部变量，所以这里 move 更常见

    let handle = thread::spawn(move || {
        println!("Hello from a thread with move, x={}!", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(move || {
        // 因为 x 变量是 i32 类型的，其实现了 Copy trait, 实际 move 的时候实际复制它的的值
        println!("Hello from a thread with move again, x={}!", x);
    });

    handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle.join().unwrap();
}

pub fn start_one_thread_with_move2() {
    let x = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Hello from a thread with move, x={:?}!", x);
    });

    handle.join().unwrap();

    // 因为 x 是 Vec<i32> 类型，其并没有实现 copy trait
    // 所以，在第一个线程中，x 的所有权已经被移动到了闭包中
    // 在第二个线程中，再次使用 x，就会报错
    // let handle = thread::spawn(move || {
    //     println!("Hello from a thread with move again, x={:?}!", x);
    // });

    // handle.join().unwrap();

    let handle = thread::spawn(|| {
        println!("Hello from a thread without move");
    });

    handle.join().unwrap();
}

pub fn test_thread_move() {
    let x = vec![1, 2, 3];

    let y = 0;

    // 闭包可能的存活时间可能比 test_thread_move 函数长
    // 而 x 和 y 的所有权是属于 test_thread_move 函数的
    // 所以，在 闭包中引用了 x 和 y，就会违反借用规则，导致编译不通过
    // 所以，需要使用 move 关键字，将 x 和 y 的所有权转移到闭包中

    // let handle = thread::spawn(|| {

    //  println!("Hello from a thread without move, x = {:?}!", x.clone());
    //     println!("Hello from a thread without move, y = {:?}!", y);
    // });
    
    let handle = thread::spawn(move || {
        println!("Hello from a thread without move, x = {:?}!", x.clone());
        println!("Hello from a thread without move, y = {:?}!", y);
    });

    handle.join().unwrap();

    // println!("no moving, x = {:?}!", x);
}
