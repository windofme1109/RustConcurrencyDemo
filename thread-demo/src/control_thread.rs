use std::thread;
use std::time::Duration;

use thread_control::*;

// 使用 thread-control 完成对线程的控制

pub fn control_thread() {

    // make_pair 方法生成一对对象 flag,control
    // flag 和 control 是相互联系的，一个发生变化，另外一个立马能感知到
    // 这里 control 交给父进程进行控制，你可以调用 stop 方法触发信号，
    // 这个时候 flag.alive() 就会变为 false
    // 如果子线程 panickled, 可以通过 control.is_interrupted() == true 来判断

    let (flag, control) = make_pair();

    let handle = thread::spawn(move || {

        while flag.alive() {

            thread::sleep(Duration::from_millis(100));

            println!("I'm alive!");
        }
    });

     thread::sleep(Duration::from_millis(1000));

     assert_eq!(control.is_done(), false);

     // 终止线程
     control.stop(); // Also you can ‘control.interrupt()‘ it
     handle.join().unwrap();

     assert_eq!(control.is_interrupted(), false);
     assert_eq!(control.is_done(), true);

     println!("This thread is stopped")
}