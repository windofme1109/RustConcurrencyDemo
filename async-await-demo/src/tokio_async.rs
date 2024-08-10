pub fn tokio_async() {

    // 创建一个异步运行时
    let rt = tokio::runtime::Runtime::new().unwrap();

    // block_on 方法在 Tokio 运行时上运行并完成一个 Future 任务，这是运行时的入口点
    // block_on 方法在当前线程上运行给定的 Future，会阻塞当前线程直到完成任务，并产生其解析结果
    // 在 block_on 方法内部生成的任务或计时器都将在运行时执行
    rt.block_on(async {
        println!("Hello from tokio!");

        // 在 Tokio 运行时（runtime）生成一个 Future
        // 将给定的 Future 生成到运行时执行器（通常是线程池）中
        // 然后线程池负责轮询 Future，直到 Future 完成
        // spawn 返回一个 JoinHandle, 所以这里调用 .await 来等待任务结束
        rt.spawn(async {
            println!("Hello from a tokio task!");
            println!("in spawn")
        })
            .await
            .unwrap();
    });

    // 使用 spawn_blocking 在运行时中执行一个普通的阻塞任务
    // 这个任务会在线程池中运行, 而不会阻塞运行时
    rt.spawn_blocking(|| println!("in spawn_blocking"));
}

// 总结一下这个例子展示的要点:
// • 在 Tokio 运行时中用 block_on 执行异步任务
// • 用 spawn 在运行时中异步执行任务
// • 用 spawn_blocking 在线程池中执行阻塞任务
// • 可以 await JoinHandle 来等待异步任务结束