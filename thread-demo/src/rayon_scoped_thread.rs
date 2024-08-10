use rayon;

// 第三方 crate rayon 也提供了和 crossbeam 类似的机制，用来创建孙线程，子子孙孙线程

pub fn rayon_scope() {
    let mut a = vec![1, 2, 3];

    let mut x = 0;

    rayon::scope(|s| {
        s.spawn(|_| {
            println!("hello from the first rayon scoped thread");

            dbg!(&a);
        });

        s.spawn(|_| {
            println!("hello from the second rayon scoped thread");

            x += a[0] + a[2];
        });

        println!("hello from the main thread");
    });

    // After the scope, we can modify and access our variables again:

    a.push(4);

    assert_eq!(x, a.len());
}

pub fn rayon_fifo_example() {
    // rayon 还提供了另外一个功能: fifo 的 scope thread
    // 比如下面一段 scope_fifo 代码：
    // point start
    rayon::scope_fifo(|s| {
        s.spawn_fifo(|s| {
            println!("s.1");
            // task s.1
            s.spawn_fifo(|s| {
                println!("s.1.1");
                // task s.1.1
                rayon::scope_fifo(|t| {
                    t.spawn_fifo(|_| {
                        // task t.1
                        println!("t.1");
                    });
                    t.spawn_fifo(|_| {
                        // task t.2
                        println!("t.2");
                    });
                });
            });
        });

        s.spawn_fifo(|s| {
            println!("s.2");
            // task s.2
        });
        // point mid
    }); // point end
}

// 上面函数中，线程并发执行的顺序类似下面的顺序：
//  | (start)
//  |
//  | (FIFO scope `s` created)
//  +--------------------+ (task s.1)
//  +-------+ (task s.2) |
//  |       |            +---+ (task s.1.1)
//  |       |            |   |
//  |       |            |   | (FIFO scope `t` created)
//  |       |            |   +----------------+ (task t.1)
//  |       |            |   +---+ (task t.2) |
//  | (mid) |            |   |   |            |
//  :       |            |   + <-+------------+ (scope `t` ends)
//  :       |            |   |
//  |<------+------------+---+ (scope `s` ends)
//  |
//  | (end)

// 在同一层作用域内，先创建的线程优先执行

// 线程的执行顺序可以认为是：s.1 先于 s.2，t.1 先于 t.2