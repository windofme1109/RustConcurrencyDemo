use std::{io, thread};
use num_cpus;
use num_threads::{num_threads};
use thread_amount;


// 并发数和当前线程数量
pub fn test_max_concurrency_count() -> io::Result<()> {
    let count = thread::available_parallelism().unwrap().get();

    println!("max count {}", count);
    assert!(count > 1_usize);

    


    // 第三方 affinity (不支持 MacOS) crate 可以提供当前的 CPU 核数:
    // let cores: Vec<usize> = (0..affinity::get_core_num()).step_by(2).collect();
    // println!("cores : {:?}", &cores);

    // 更多的场景下，我们使用 num_cpus crate 获取 CPU 的核数(逻辑核):
    let num = num_cpus::get();

    println!("current cpu core count is {}", num);

    // let thread_count_of_current_process = num_threads::num_threads().unwrap().get();
    // let thread_count_of_current_process2 = num_threads::num_threads().unwrap().get();
    // println!("thread count of current process is {}", thread_count_of_current_process);



    // 如果想获得当前进程的线程数，比如在一些性能监控收集指标的时候，你可以使 用 num_threads crate
    // 实际测试 num_threads 不支持 windows，你可以使用 thread-amount 代替 
    if let Some(count) = num_threads::num_threads() { 
        println!("num_threads: {}", count);
    } else {
        println!("num_threads: not supported");
    
    }
    
    let count = thread_amount::thread_amount();
    
    if !count.is_none() {
        println!("thread_amount: {}", count.unwrap());
    }

    let count = num_cpus::get();
    println!("num_cpus: {}", count);

    Ok(())

}

