use std::thread;
use crossbeam::thread as crossbeam_thread;
use std::sync::mpsc::channel;
use std::rc::Rc;
use send_wrapper::SendWrapper;
// 跨线程的变量必须实现 Send, 否则不允许在跨线程使用
// 例如：

pub fn wrong_send() {

    // 因为 Rc 没有实现 Send trait, 所以它不能直接在线程间使用
    // 因为两个线程使用的 Rc 指向相同的引用计数值，它们同时更新这个引用计数
    // 并且没有使用原子操作，可能会导致意想不到的行为。可以通过 Arc 类型替换 Rc 类型

    // let counter = Rc::new(42);
    //
    // let (sender, receiver) = channel();
    //
    // let _t = thread::spawn(move || {
    //     sender.send(counter).unwrap();
    // });
    //
    //  let value = receiver.recv().unwrap();
    //
    //  println!("received from the main thread: {}", value);

}

// 使用第三方 crate send_wrapper
// 允许我们在线程之间移动非 Send 类型，只要我们仅从原始线程内访问所包含的值即可

// https://crates.io/crates/send_wrapper/0.6.0


// This import is important. It allows you to unwrap the value using deref(),
// deref_mut() or Deref coercion.
use std::ops::{Deref, DerefMut};
pub fn send_wrapper() {
    let wrapped_value = SendWrapper::new(Rc::new(42));
    let (sender, receiver) = channel();
    let _t = thread::spawn(move || {
        sender.send(wrapped_value).unwrap();
    });
    let wrapped_value = receiver.recv().unwrap();

    // 在主线程中定义的 wrapper_value，所以只能在主线程中访问
    let value = wrapped_value.deref();

    println!("received from the main thread: {}", value);
}